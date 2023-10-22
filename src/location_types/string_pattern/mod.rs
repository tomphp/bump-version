use pattern::Pattern;
use std::path::Path;
use update_file::update_file;

mod config;
mod pattern;
mod substitute_pattern;
mod update_file;

pub(crate) use config::Config;

pub(crate) fn replace_version_with_string_pattern(
    location_config: &Config,
    version: &str,
) -> anyhow::Result<()> {
    let the_pattern = Pattern::new(&location_config.pattern)?;

    update_file(Path::new(&location_config.file), |contents| {
        substitute_pattern::substitute_pattern(&the_pattern, contents, r"\d+\.\d+\.\d+", version)
            .expect("TODO handler error properly")
    })
}
