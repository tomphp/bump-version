use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) struct Config {
    pub(crate) file: String,
    pub(crate) pattern: String,
}
