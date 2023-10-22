use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, Write};
use std::path::Path;

use anyhow::anyhow;

pub fn update_file<F>(path: &Path, update: F) -> anyhow::Result<()>
where
    F: FnOnce(&str) -> String,
{
    perform_update(path, update).map_err(|err: anyhow::Error| {
        anyhow!("Failed to update {}: {}", path.display(), err.to_string())
    })
}

fn perform_update<F>(path: &Path, update: F) -> Result<(), anyhow::Error>
where
    F: FnOnce(&str) -> String,
{
    if !path.exists() {
        return Err(anyhow!("File does not exist"));
    }

    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_contents = update(&contents);

    file.seek(io::SeekFrom::Start(0))?;
    file.write_all(new_contents.as_bytes())?;
    file.set_len(new_contents.len() as u64)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::NamedTempFile;

    use super::update_file;

    #[test]
    fn update_file_updates_the_contents_of_the_file() {
        let file = NamedTempFile::new().expect("Failed to create temporary file");
        fs::write(
            file.path(),
            "This is a temporary file which includes <THIS> placeholder",
        )
        .expect("Failed to write test file");

        update_file(file.path(), |content| {
            content.replace("<THIS>", "a substituted")
        })
        .expect("Failed to edit file");

        let result = fs::read_to_string(file.path()).expect("Failed to read edited file");
        assert_eq!(
            result,
            "This is a temporary file which includes a substituted placeholder"
        );
    }

    #[test]
    fn update_file_returns_error_when_file_does_not_exist() {
        let result = update_file(
            Path::new("this-file-does-not-exist.txt"),
            std::borrow::ToOwned::to_owned,
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to update this-file-does-not-exist.txt: File does not exist"
        );
    }
}
