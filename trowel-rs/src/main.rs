use std::{error::Error, ffi::OsStr, fs, io, path::{Path, PathBuf}, process::{Command, Stdio}};

use clap::{command, Parser};
use model::trowel_diff::TrowelDiff;
use ratatui::{
    backend::Backend, crossterm::event::{self}, Terminal
};
use state::app_state::{AppState, Lifecycle};
use tempfile::NamedTempFile;

mod state;
mod ui;
mod model;
mod widget;

use crate::{
    ui::ui,
    model::tf_plan::TfPlan,
};

#[derive(Parser, Debug)]
#[command(version, about = "A TUI for working with OpenTofu and Terraform", long_about = None)]
struct Args {
    #[arg(short, long)]
    plan_file: Option<PathBuf>,
    #[arg(short, long, default_value = "tofu")]
    binary: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    let plan_file = args.plan_file;
    let binary = args.binary;
    let (diff, text_plan) = match plan_file {
        Some(p) => generate_diff_from_plan(p, &binary),
        None => {
            let (diff, plan) = generate_diff(&binary)?;
            Ok((diff, Some(plan)))
        }
    }?;

    let mut terminal = ratatui::init();
    let mut app = AppState::new(diff, text_plan);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn generate_binary_plan(binary: &String) -> Result<NamedTempFile, io::Error> {
    let file = NamedTempFile::new()?;
    match file.path().to_str() {
        Some(p) => {
            let mut cmd = Command::new(binary)
                .arg("plan")
                .arg("-out")
                .arg(p)
                .stderr(Stdio::inherit())
                .spawn()?;
            let _ = cmd.wait()?;
            Ok(file)
        },
        None => {
            Err(io::Error::new(io::ErrorKind::NotFound, "No string representation available for tempfile path"))
        }
    }
}

type TextPlan = String;

fn generate_diff(binary: &String) -> Result<(TrowelDiff, TextPlan), io::Error> {
    // NOTE: NamedTempFile automatically deletes its tempfile when dropped via its destructor, and so should be dropped explicitly
    let binary_tempfile = generate_binary_plan(binary)?;
    let (diff, plan) = generate_diff_binary(binary_tempfile.path().to_path_buf(), binary)?;
    drop(binary_tempfile);
    Ok((diff, plan))
}

fn is_json_file(path: &Path) -> bool {
    let extension = path.extension().and_then(OsStr::to_str);
    matches!(extension, Some("json"))
}

fn generate_diff_from_plan(plan_file: PathBuf, binary: &String) -> Result<(TrowelDiff, Option<TextPlan>), io::Error> {
    if is_json_file(plan_file.as_path()) {
        let json_data = fs::read_to_string(&plan_file)?;
        Ok((generate_diff_json(json_data)?, None))
    } else {
        let (diff, plan) = generate_diff_binary(plan_file, binary)?;
        Ok((diff, Some(plan)))
    }
}

fn generate_diff_json(json_data: String) -> Result<TrowelDiff, io::Error> {
    let parsed: TfPlan = serde_json::from_str(&json_data)?;
    TrowelDiff::from_tf_plan(&parsed)
}

fn generate_diff_binary(plan_file: PathBuf, binary: &String) -> Result<(TrowelDiff, TextPlan), io::Error> {
    let text_plan = generate_text_plan(&plan_file, binary)?;
    let output = Command::new(binary)
        .arg("show")
        .arg("-json")
        .arg(plan_file)
        .output()?;
    match std::str::from_utf8(&output.stdout) {
        Ok(out) => Ok((generate_diff_json(out.to_string())?, text_plan)),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse JSON plan stdout into UTF-8")),
    }
}

fn generate_text_plan(binary_plan: &PathBuf, binary: &String) -> Result<TextPlan, io::Error> {
    let output = Command::new(binary)
        .arg("show")
        .arg("-no-color")
        .arg(binary_plan)
        .output()?;
    match std::str::from_utf8(&output.stdout) {
        Ok(out) => Ok(out.to_string()),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse text plan stdout into UTF-8")),
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    loop {
        match app.lifecycle {
            Lifecycle::Running => {
                terminal.draw(|f| ui(f, app))?;
                app.process_event(event::read()?);
            },
            Lifecycle::Quit => {
                return Ok(())
            },
        }
    }
}