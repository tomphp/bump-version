use anyhow::anyhow;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern(String);

impl Pattern {
    pub(crate) fn new(string: &str) -> anyhow::Result<Self> {
        if !string.contains("{{version}}") {
            return Err(
                anyhow!("String pattern \"{string}\" does not contain the required {{{{version}}}} placeholder")
            );
        }
        Ok(Self(string.to_owned()))
    }

    pub(crate) fn substituted_with(&self, version: &str) -> String {
        self.0.replace("{{version}}", version)
    }

    pub(crate) fn to_regex(&self, version_pattern: &str) -> anyhow::Result<Regex> {
        Regex::new(&self.0.replace("{{version}}", version_pattern))
            .map_err(|err| anyhow!("Version pattern is not a valid regex: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn new_returns_instance() {
        assert_eq!(
            Pattern::new("current version: {{version}}").expect("Failed to create pattern"),
            Pattern("current version: {{version}}".to_owned())
        );
    }

    #[test]
    fn new_returns_error_when_placeholder_is_missing() {
        assert_eq!(
            Pattern::new("missing version").expect_err("Expected an error").to_string(),
            "String pattern \"missing version\" does not contain the required {{version}} placeholder"
        );
    }

    #[test]
    fn substituted_with_returns_the_pattern_with_a_substituted_version() {
        assert_eq!(
            Pattern::new("current version: {{version}}")
                .map(|p| p.substituted_with("1.2.3"))
                .expect("Failed to substitute version"),
            "current version: 1.2.3"
        );
    }

    #[test]
    fn to_regex_returns_regex_with_version_pattern() {
        assert_eq!(
            Pattern::new("current version: {{version}}")
                .and_then(|p| p.to_regex(r"\d+\.\d+\.\d+"))
                .map(|r| r.to_string())
                .expect("Failed to convert to regex"),
            r"current version: \d+\.\d+\.\d+"
        );
    }

    #[test]
    fn to_regex_returns_an_error_when_version_pattern_is_an_invalid_regex() {
        let result = Pattern::new("current version: {{version}}").and_then(|p| p.to_regex(r"["));
        assert_eq!(result.expect_err("Failed to create pattern").to_string(), "Version pattern is not a valid regex: regex parse error:\n    current version: [\n                     ^\nerror: unclosed character class");
    }
}
