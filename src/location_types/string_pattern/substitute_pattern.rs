use super::pattern::Pattern;

pub fn substitute_pattern(
    pattern: &Pattern,
    content: &str,
    version_pattern: &str,
    version: &str,
) -> anyhow::Result<String> {
    let regex = pattern.to_regex(version_pattern)?;
    let replacement = pattern.substituted_with(version);
    Ok(regex.replace_all(content, replacement).to_string())
}

#[cfg(test)]
mod tests {
    use super::{substitute_pattern, Pattern};

    #[test]
    fn substitute_pattern_returns_string_with_version_substituted() {
        let pattern =
            Pattern::new("current version is {{version}}").expect("Failed to create pattern");
        let content = "The app's current version is 1.3.3";
        let version = "1.4.0";
        let version_pattern = r"\d+\.\d+\.\d+";

        assert_eq!(
            substitute_pattern(&pattern, content, version_pattern, version)
                .expect("Failed to substitute pattern"),
            "The app's current version is 1.4.0".to_owned()
        );
    }

    #[test]
    fn substitute_pattern_returns_an_error_when_version_pattern_is_a_bad_regex() {
        let pattern =
            Pattern::new("current version is {{version}}").expect("Failed to create pattern");
        let content = "The app's current version is 1.3.3";
        let version = "1.4.0";
        let version_pattern = r"[";

        assert_eq!(
            substitute_pattern(&pattern, content, version_pattern, version).expect_err("Expected an error").to_string(),
            "Version pattern is not a valid regex: regex parse error:\n    current version is [\n                       ^\nerror: unclosed character class"
        );
    }
}
