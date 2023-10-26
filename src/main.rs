use std::pin::Pin;
use std::process;

use async_stream::stream;
use clap::{Parser, Subcommand};
use futures::{Stream, StreamExt};

use config::Location;
use Location::{Cargo, StringPattern};

use crate::config::Config;

mod config;
mod location_types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Updates the version in all known locations
    Update {
        /// The new value to be used when representing a value
        #[arg()]
        version: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { version } => update_command(version).await,
    }
}

#[derive(Debug)]
enum Event {
    Errored(String),
    UpdateStarted(usize, String),
    UpdateSucceeded(usize),
    UpdateFailed(usize, String),
}

async fn update_command(version: &str) {
    const CONFIG_FILE: &str = "versioned-files.yml";

    let config = Config::from_file(CONFIG_FILE);

    let mut streams = config.map(|config| config.locations).map_or_else(
        |err: anyhow::Error| -> Pin<Box<dyn Stream<Item=Event>>> {
            Box::pin(stream! { yield Event::Errored(err.to_string()) })
        },
        |locations| -> Pin<Box<dyn Stream<Item=Event>>> {
            Box::pin(futures::stream::select_all(locations.into_iter().map(
                |location| update_location(1, version.to_string(), location),
            )))
        },
    );

    while let Some(event) = streams.next().await {
        match event {
            Event::Errored(err) => {
                eprintln!("ERROR: {err}");
                process::exit(1);
            }
            Event::UpdateStarted(_, file) => {
                print!("Updating {file}...");
            }
            Event::UpdateSucceeded(_) => {
                println!("success");
            }
            Event::UpdateFailed(_, message) => {
                println!("failed");
                println!("ERROR: {message}");
                process::exit(1);
            }
        }
    }
}

fn update_location(
    id: usize,
    version: String,
    location: Location,
) -> Pin<Box<dyn Stream<Item=Event>>> {
    match location {
        StringPattern(location_config) => {
            Box::pin(update_string_pattern_location(id, version, location_config))
        }
        Cargo => Box::pin(update_cargo_location(id, version)),
    }
}

fn update_cargo_location(id: usize, version: String) -> impl Stream<Item=Event> {
    stream! {
        yield Event::UpdateStarted(id, "Cargo.toml".to_string());
        match location_types::cargo::update_cargo_version(&version) {
            Ok(()) => {
              yield Event::UpdateSucceeded(id);
              yield Event::UpdateStarted(id, "Cargo.lock".to_string());
              yield Event::UpdateSucceeded(id);
            }
            Err(err) => {
                yield Event::UpdateFailed(id, err.to_string());
            }
        }
    }
}

fn update_string_pattern_location(
    id: usize,
    version: String,
    location_config: location_types::string_pattern::Config,
) -> impl Stream<Item=Event> {
    stream! {
        yield Event::UpdateStarted(id, location_config.file.clone());
        match location_types::string_pattern::replace_version_with_string_pattern(
            &location_config,
            &version,
        ) {
            Ok(()) => yield Event::UpdateSucceeded(id),
            Err(err) => yield Event::UpdateFailed(id, err.to_string()),
        }
    }
}
