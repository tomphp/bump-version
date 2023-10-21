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
    static CONFIG_FILE: &str = "versioned-files.yml";

    let cli = Cli::parse();

    match &cli.command {
        Commands::Bump { version: _ } => {
            if !Path::new(CONFIG_FILE).exists() {
                eprintln!("ERROR: No {CONFIG_FILE} file found.");
                process::exit(1);
            }
        }
    }
}
