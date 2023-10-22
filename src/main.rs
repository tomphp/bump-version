use std::fs::File;
use std::path::Path;
use std::process;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::Location::StringPattern;

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

#[derive(Debug, Deserialize, PartialEq)]
enum Location {
    #[serde(rename = "string-pattern")]
    StringPattern(location_types::string_pattern::Config),
}

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    locations: Vec<Location>,
}

fn check_file_exists(path: &str) -> anyhow::Result<&str> {
    Some(path)
        .filter(|p| Path::new(p).exists())
        .ok_or(anyhow!("No {path} file found."))
}

fn open_file(path: &str) -> anyhow::Result<File> {
    File::open(path).map_err(|err| anyhow!("Failed to read {path}: {err}"))
}

fn parse_config(file: File) -> anyhow::Result<Config> {
    serde_yaml::from_reader(file).map_err(|err| anyhow!("Invalid config file schema: {err}"))
}

fn read_config(path: &str) -> anyhow::Result<Config> {
    check_file_exists(path)
        .and_then(open_file)
        .and_then(parse_config)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { version } => update_command(version),
    }
}

fn update_command(version: &str) {
    const CONFIG_FILE: &str = "versioned-files.yml";

    match read_config(CONFIG_FILE) {
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
