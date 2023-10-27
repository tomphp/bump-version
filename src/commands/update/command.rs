use futures::StreamExt;

use crate::config::Location::{Cargo, StringPattern};
use crate::config::{Config, Location};
use crate::formatter::Formatter;
use crate::location_types;

use super::state::State;
use super::{event, Updater};

pub struct Command<'a, F: Formatter> {
    formatter: &'a F,
}

impl<'a, F: Formatter + Send + Sync> Command<'a, F> {
    pub const fn new(formatter: &'a F) -> Self {
        Command { formatter }
    }

    pub async fn execute(&self, version: &str) -> Result<(), anyhow::Error> {
        const CONFIG_FILE: &str = "versioned-files.yml";

        let config = Config::from_file(CONFIG_FILE);

        let mut streams = config
            .map(|config| config.locations)
            .map(|locations| update_locations(version, locations))?;

        let mut state = State::new(self.formatter);

        while let Some(event) = streams.next().await {
            state.update_event(&event);
        }

        state.as_result()
    }
}

fn update_locations(version: &str, locations: Vec<Location>) -> event::Stream {
    Box::pin(futures::stream::select_all(
        locations
            .into_iter()
            .map(|location| update_location(version.to_string(), location)),
    ))
}

fn update_location(version: String, location: Location) -> event::Stream {
    create_updater(location).update(version)
}

fn create_updater(location: Location) -> Box<dyn Updater> {
    match location {
        StringPattern(config) => Box::new(location_types::string_pattern::Location::new(config)),
        Cargo => Box::new(location_types::cargo::Location::new()),
    }
}
