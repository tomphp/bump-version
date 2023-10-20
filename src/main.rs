use std::path::Path;
use std::process;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Bump {
        #[arg()]
        version: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bump { version: _ } => {
            if !Path::new("bump-version.yml").exists() {
                eprintln!("ERROR: No bump-version.yml file found.");
                process::exit(1);
            }
        }
    }
}
