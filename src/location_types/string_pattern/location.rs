use std::path::Path;

use async_stream::stream;

use crate::commands::update::{EventStream, LifeCycle, Updater};

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
            let life_cycle = LifeCycle::new(&config.file);
            yield life_cycle.start();
            match replace_version(&config, &version) {
                Ok(()) => yield life_cycle.succeed(),
                Err(err) => yield life_cycle.fail(&err.to_string()),
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
