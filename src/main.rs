use clap::{Parser, Subcommand};
use std::process;

use config::{Config, Location};
use Location::{Cargo, StringPattern};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { version } => update_command(version),
    }
}

enum Event {
    Errored(String),
    UpdateStarted(usize, String),
    UpdateSucceeded(usize),
    UpdateFailed(usize, String),
}

fn update_command(version: &str) {
    const CONFIG_FILE: &str = "versioned-files.yml";

    let events: Vec<Event> = Config::from_file(CONFIG_FILE)
        .map(|config| {
            config
                .locations
                .iter()
                .enumerate()
                .flat_map(|(id, location)| update_location(id, version, location))
                .collect::<Vec<_>>()
        })
        .or_else(|err| Ok::<Vec<Event>, ()>(vec![Event::Errored(err.to_string())]))
        .unwrap();

    for event in events {
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

fn update_location(id: usize, version: &str, location: &Location) -> Vec<Event> {
    match location {
        StringPattern(location_config) => {
            vec![
                Event::UpdateStarted(id, location_config.file.clone()),
                match location_types::string_pattern::replace_version_with_string_pattern(
                    location_config,
                    version,
                ) {
                    Ok(()) => Event::UpdateSucceeded(id),
                    Err(err) => Event::UpdateFailed(id, err.to_string()),
                },
            ]
        }
        Cargo => vec![
            vec![Event::UpdateStarted(id, "Cargo.toml".to_string())],
            match location_types::cargo::update_cargo_version(version) {
                Ok(()) => {
                    vec![
                        Event::UpdateSucceeded(id),
                        Event::UpdateStarted(id, "Cargo.lock".to_string()),
                        Event::UpdateSucceeded(id),
                    ]
                }
                Err(err) => {
                    vec![Event::UpdateFailed(id, err.to_string())]
                }
            },
        ]
        .into_iter()
        .flatten()
        .collect(),
    }
}
