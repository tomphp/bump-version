use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
    pub(crate) file: String,
    pub(crate) pattern: String,
}
