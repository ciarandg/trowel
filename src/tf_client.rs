use std::io;
use std::path::PathBuf;

use tempfile::NamedTempFile;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};

#[derive(Clone)]
pub struct TfClient {
    binary: String,
}

impl TfClient {
    pub fn new(binary: String) -> Self {
        Self { binary }
    }

    pub async fn plan(&self, tx: mpsc::Sender<String>) -> Result<NamedTempFile, io::Error> {
        let tempfile = NamedTempFile::new()?;
        let mut child = tokio::process::Command::new(&self.binary)
            .arg("plan")
            .arg("-no-color")
            .arg("-out")
            .arg(tempfile.path())
            .stderr(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to take stdout for plan process",
        ))?;
        let mut reader = BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await? {
            tx.send(line).await.ok();
        }

        Ok(tempfile)
    }

    pub fn show_as_json(&self, binary_plan_file: &PathBuf) -> Result<String, io::Error> {
        let output = std::process::Command::new(&self.binary)
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
        let output = std::process::Command::new(&self.binary)
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
