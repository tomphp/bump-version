use pattern::Pattern;
use std::path::Path;
use update_file::update_file;

mod pattern;
mod substitute_pattern;
mod update_file;

pub(crate) fn replace_version_with_string_pattern(
    path: &Path,
    pattern: &str,
    version: &str,
) -> anyhow::Result<()> {
    let the_pattern = Pattern::new(pattern)?;

    update_file(path, |contents| {
        substitute_pattern::substitute_pattern(&the_pattern, contents, r"\d+\.\d+\.\d+", version)
            .expect("TODO handler error properly")
    })
}
