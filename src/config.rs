use crate::location_types;
use anyhow::anyhow;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum Location {
    #[serde(rename = "string-pattern")]
    StringPattern(location_types::string_pattern::Config),
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
    pub(crate) locations: Vec<Location>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        check_file_exists(path)
            .and_then(open_file)
            .and_then(parse_config)
    }
}

fn check_file_exists(path: &str) -> anyhow::Result<&str> {
    Some(path)
        .filter(|p| Path::new(p).exists())
        .ok_or(anyhow!("No {path} file found."))
}

fn open_file(path: &str) -> anyhow::Result<File> {
    File::open(path).map_err(|err| anyhow!("Failed to read {path}: {err}"))
}

fn parse_config(file: File) -> anyhow::Result<Config> {
    serde_yaml::from_reader(file).map_err(|err| anyhow!("Invalid config file schema: {err}"))
}
