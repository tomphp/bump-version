use pattern::Pattern;
use std::path::Path;
use update_file::update_file;

mod config;
mod pattern;
mod substitute_pattern;
mod update_file;

pub use config::Config;

pub fn replace_version(location_config: &Config, version: &str) -> anyhow::Result<()> {
    let the_pattern = Pattern::new(&location_config.pattern)?;

    update_file(Path::new(&location_config.file), |contents| {
        substitute_pattern::substitute_pattern(&the_pattern, contents, r"\d+\.\d+\.\d+", version)
            .expect("TODO handler error properly")
    })
}
