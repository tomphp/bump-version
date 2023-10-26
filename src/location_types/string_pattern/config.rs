use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Config {
    pub(crate) file: String,
    pub(crate) pattern: String,
}
