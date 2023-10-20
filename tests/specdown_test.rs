use assert_cmd::output::OutputResult;
use assert_cmd::Command;
use std::env;

fn assert_ok(result: &OutputResult) {
    let output = match result {
        Ok(out) => out,
        Err(err) => err.as_output().unwrap(),
    };

    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));

    assert!(result.is_ok());
}

#[test]
fn test_readme() {
    let result = Command::new("specdown")
        .arg("run")
        .arg("--temporary-workspace-dir")
        .arg("--add-path")
        .arg(env::current_dir().unwrap().join("target/release").to_str().unwrap())
        .arg("README.md")
        .ok();

    assert_ok(&result);
}
