use super::event::Event;
use super::state::State;
use crate::config::Location::{Cargo, StringPattern};
use crate::config::{Config, Location};
use crate::formatter::plain::Plain;
use crate::location_types;
use async_stream::stream;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use uuid::Uuid;

pub async fn execute(version: &str) -> Result<(), anyhow::Error> {
    const CONFIG_FILE: &str = "versioned-files.yml";

    let config = Config::from_file(CONFIG_FILE);

    let mut streams = config.map(|config| config.locations).map(
        |locations| -> Pin<Box<dyn Stream<Item = Event> + Send>> {
            Box::pin(futures::stream::select_all(
                locations
                    .into_iter()
                    .map(|location| update_location(version.to_string(), location)),
            ))
        },
    )?;

    let mut formatter = Plain {};
    let mut state = State::new(&mut formatter);

    while let Some(event) = streams.next().await {
        state.update_event(&event);
    }

    state.as_result()
}

fn update_location(
    version: String,
    location: Location,
) -> Pin<Box<dyn Stream<Item = Event> + Send>> {
    match location {
        StringPattern(location_config) => {
            Box::pin(update_string_pattern_location(version, location_config))
        }
        Cargo => Box::pin(update_cargo_location(version)),
    }
}

fn update_cargo_location(version: String) -> impl Stream<Item = Event> {
    stream! {
        let toml_id = Uuid::new_v4();
        let lock_id = Uuid::new_v4();
        yield Event::Started(toml_id, "Cargo.toml".to_string());
        match location_types::cargo::update_cargo_version(&version) {
            Ok(()) => {
              yield Event::Succeeded(toml_id);
              yield Event::Started(lock_id, "Cargo.lock".to_string());
              yield Event::Succeeded(lock_id);
            }
            Err(err) => {
                yield Event::Failed(toml_id, err.to_string());
            }
        }
    }
}

fn update_string_pattern_location(
    version: String,
    location_config: location_types::string_pattern::Config,
) -> impl Stream<Item = Event> {
    stream! {
        let id = Uuid::new_v4();
        yield Event::Started(id, location_config.file.clone());
        match location_types::string_pattern::replace_version(
            &location_config,
            &version,
        ) {
            Ok(()) => yield Event::Succeeded(id),
            Err(err) => yield Event::Failed(id, err.to_string()),
        }
    }
}
