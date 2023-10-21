use pattern::Pattern;

mod pattern;
mod substitute_pattern;

pub(crate) fn do_it() {
    let _ = substitute_pattern::substitute_pattern(
        &Pattern::new("current version: {{version}}").unwrap(),
        "the current version is: 1.2.3!",
        r"\d+\.\d+\.\d+",
        "2.0.0",
    );
}
