use crate::app_state::AppState;
use crate::config::Location::{Cargo, StringPattern};
use crate::config::{Config, Location};
use crate::formatter::plain::Plain;
use crate::location_types;
use crate::update_event::UpdateEvent;
use async_stream::stream;
use clap::Subcommand;
use futures::{Stream, StreamExt};
use std::pin::Pin;

#[derive(Subcommand)]
pub enum Command {
    /// Updates the version in all known locations
    Update {
        /// The new value to be used when representing a value
        #[arg()]
        version: String,
    },
}

impl Command {
    pub(crate) async fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Self::Update { version } => update_command(version).await?,
        }

        Ok(())
    }
}

async fn update_command(version: &str) -> Result<(), anyhow::Error> {
    const CONFIG_FILE: &str = "versioned-files.yml";

    let config = Config::from_file(CONFIG_FILE);

    let mut streams = config.map(|config| config.locations).map(
        |locations| -> Pin<Box<dyn Stream<Item = UpdateEvent> + Send>> {
            Box::pin(futures::stream::select_all(locations.into_iter().map(
                |location| update_location(1, version.to_string(), location),
            )))
        },
    )?;

    let mut formatter = Plain {};
    let mut state = AppState::new(&mut formatter);

    while let Some(event) = streams.next().await {
        state.update_event(&event);
    }

    state.as_result()
}

fn update_location(
    id: usize,
    version: String,
    location: Location,
) -> Pin<Box<dyn Stream<Item = UpdateEvent> + Send>> {
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
        match location_types::string_pattern::replace_version(
            &location_config,
            &version,
        ) {
            Ok(()) => yield UpdateEvent::Succeeded(id),
            Err(err) => yield UpdateEvent::Failed(id, err.to_string()),
        }
    }
}
