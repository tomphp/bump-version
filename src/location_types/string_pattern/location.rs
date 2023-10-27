use std::path::Path;

use async_stream::stream;
use uuid::Uuid;

use crate::commands::update::{Event, EventStream, Updater};

use super::config::Config;
use super::pattern::Pattern;
use super::substitute_pattern::substitute_pattern;
use super::update_file::update_file;

pub struct Location {
    config: Config,
}

impl Location {
    pub const fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Updater for Location {
    fn update(&self, version: String) -> EventStream {
        let config = self.config.clone();

        Box::pin(stream! {
            let id = Uuid::new_v4();
            yield Event::Started(id, config.file.clone());
            match replace_version(&config, &version) {
                Ok(()) => yield Event::Succeeded(id),
                Err(err) => yield Event::Failed(id, err.to_string()),
            }
        })
    }
}

pub fn replace_version(config: &Config, version: &str) -> anyhow::Result<()> {
    let the_pattern = Pattern::new(&config.pattern)?;

    update_file(Path::new(&config.file), |contents| {
        substitute_pattern(&the_pattern, contents, r"\d+\.\d+\.\d+", version)
            .expect("TODO handler error properly")
    })
}
