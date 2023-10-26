use std::pin::Pin;

use anyhow::anyhow;

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
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { version } => update_command(version).await?,
    };

    Ok(())
}

#[derive(Debug)]
enum UpdateEvent {
    Started(usize, String),
    Succeeded(usize),
    Failed(usize, String),
}

async fn update_command(version: &str) -> Result<(), anyhow::Error> {
    const CONFIG_FILE: &str = "versioned-files.yml";

    let config = Config::from_file(CONFIG_FILE);

    let mut streams = config.map(|config| config.locations).map(
        |locations| -> Pin<Box<dyn Stream<Item = UpdateEvent>>> {
            Box::pin(futures::stream::select_all(locations.into_iter().map(
                |location| update_location(1, version.to_string(), location),
            )))
        },
    )?;

    let mut result = Ok(());

    while let Some(event) = streams.next().await {
        match event {
            UpdateEvent::Started(_, file) => {
                print!("Updating {file}...");
            }
            UpdateEvent::Succeeded(_) => {
                println!("success");
            }
            UpdateEvent::Failed(_, message) => {
                println!("failed");
                println!("Error: {message}");
                result = Err(anyhow!(message));
            }
        }
    }

    result
}

fn update_location(
    id: usize,
    version: String,
    location: Location,
) -> Pin<Box<dyn Stream<Item = UpdateEvent>>> {
    match location {
        StringPattern(location_config) => {
            Box::pin(update_string_pattern_location(id, version, location_config))
        }
        Cargo => Box::pin(update_cargo_location(id, version)),
    }
}

fn update_cargo_location(id: usize, version: String) -> impl Stream<Item = UpdateEvent> {
    stream! {
        yield UpdateEvent::Started(id, "Cargo.toml".to_string());
        match location_types::cargo::update_cargo_version(&version) {
            Ok(()) => {
              yield UpdateEvent::Succeeded(id);
              yield UpdateEvent::Started(id, "Cargo.lock".to_string());
              yield UpdateEvent::Succeeded(id);
            }
            Err(err) => {
                yield UpdateEvent::Failed(id, err.to_string());
            }
        }
    }
}

fn update_string_pattern_location(
    id: usize,
    version: String,
    location_config: location_types::string_pattern::Config,
) -> impl Stream<Item = UpdateEvent> {
    stream! {
        yield UpdateEvent::Started(id, location_config.file.clone());
        match location_types::string_pattern::replace_version_with_string_pattern(
            &location_config,
            &version,
        ) {
            Ok(()) => yield UpdateEvent::Succeeded(id),
            Err(err) => yield UpdateEvent::Failed(id, err.to_string()),
        }
    }
}
