use std::process;

use clap::{Parser, Subcommand};
use config::{Config, Location};

use config::Location::StringPattern;

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

fn update_command(version: &str) {
    const CONFIG_FILE: &str = "versioned-files.yml";

    match Config::from_file(CONFIG_FILE) {
        Ok(config) => {
            for location in &*config.locations {
                match update_location(version, location) {
                    Ok(()) => {}
                    Err(err) => {
                        println!("ERROR: {err}");
                        process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("ERROR: {err}");
            process::exit(1);
        }
    }
}

fn update_location(version: &str, location: &Location) -> anyhow::Result<()> {
    match location {
        StringPattern(location_config) => {
            print!("Updating {}...", location_config.file);
            match location_types::string_pattern::replace_version_with_string_pattern(
                location_config,
                version,
            ) {
                Ok(()) => {
                    println!("success");
                    Ok(())
                }
                Err(err) => {
                    println!("failed");
                    Err(err)
                }
            }
        }
    }
}
