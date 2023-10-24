use anyhow::anyhow;
use std::io::Read;
use std::process::{Command, Stdio};

pub(crate) fn update_cargo_version(version: &str) -> anyhow::Result<()> {
    check_cargo_is_installed()?;
    install_cargo_edit()?;
    update_version(version)?;
    Ok(())
}

fn check_cargo_is_installed() -> anyhow::Result<()> {
    let command = Command::new("cargo")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(
            "Failed to execute cargo, please make sure it is installed: {err}"
        )),
    }
}

fn install_cargo_edit() -> anyhow::Result<()> {
    let command = Command::new("cargo")
        .arg("install")
        .arg("cargo-edit")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("Failed install cargo-edit: {err}")),
    }
}

fn update_version(version: &str) -> anyhow::Result<()> {
    let command = Command::new("cargo")
        .arg("set-version")
        .arg(version)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match command {
        Ok(mut child) => {
            let mut stdout = String::new();
            let mut stderr = String::new();

            child
                .stdout
                .as_mut()
                .unwrap()
                .read_to_string(&mut stdout)
                .unwrap();
            child
                .stderr
                .as_mut()
                .unwrap()
                .read_to_string(&mut stderr)
                .unwrap();

            let exit_status = child
                .wait()
                .expect("Failed to wait for the command to finish");

            if exit_status.success() {
                Ok(())
            } else {
                Err(anyhow!(
                    "Failed to set version (cargo-edit exit code: {exit_status}): {stderr}"
                ))
            }
        }
        Err(err) => Err(anyhow!("Failed to update Cargo version: {err}")),
    }
}
