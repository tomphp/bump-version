use anyhow::anyhow;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::process;

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

#[derive(Deserialize)]
struct Location {
    #[serde(alias = "type")]
    location_type: String,
    file: String,
    pattern: String,
}

#[derive(Deserialize)]
struct Config {
    locations: Box<[Location]>,
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
    const CONFIG_FILE: &str = "versioned-files.yml";

    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { version: _ } => match read_config(CONFIG_FILE) {
            Ok(config) => {
                for location in &*config.locations {
                    location_types::string_pattern::do_it();
                    println!(
                        "type: {} / file: {} / pattern: {}",
                        location.location_type, location.file, location.pattern
                    );
                }
            }
            Err(err) => {
                eprintln!("ERROR: {err}");
                process::exit(1);
            }
        },
    }
}
