use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use tempfile::NamedTempFile;

pub struct TfClient {
    binary: String,
}

impl TfClient {
    pub fn new(binary: String) -> Self {
        Self { binary }
    }

    pub fn plan(&self) -> Result<NamedTempFile, io::Error> {
        let file = NamedTempFile::new()?;
        match file.path().to_str() {
            Some(p) => {
                let mut cmd = Command::new(&self.binary)
                    .arg("plan")
                    .arg("-out")
                    .arg(p)
                    .stderr(Stdio::inherit())
                    .spawn()?;
                let _ = cmd.wait()?;
                Ok(file)
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No string representation available for tempfile path",
            )),
        }
    }

    pub fn show_as_json(&self, binary_plan_file: &PathBuf) -> Result<String, io::Error> {
        let output = Command::new(&self.binary)
            .arg("show")
            .arg("-json")
            .arg(binary_plan_file)
            .output()?;
        match std::str::from_utf8(&output.stdout) {
            Ok(out) => Ok(out.to_owned()),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to parse JSON plan stdout into UTF-8",
            )),
        }
    }

    pub fn show_as_text(&self, binary_plan_file: &PathBuf) -> Result<String, io::Error> {
        let output = Command::new(&self.binary)
            .arg("show")
            .arg("-no-color")
            .arg(binary_plan_file)
            .output()?;
        match std::str::from_utf8(&output.stdout) {
            Ok(out) => Ok(out.to_owned()),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to parse text plan stdout into UTF-8",
            )),
        }
    }
}
