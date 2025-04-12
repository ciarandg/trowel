use std::{
    error::Error,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use clap::{Parser, command};
use model::trowel_diff::TrowelDiff;
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    crossterm::event::{self},
};
use state::app_state::{AppState, Lifecycle};
use tempfile::NamedTempFile;
use tf_client::TfClient;
use widget::app_view::AppView;

mod model;
mod state;
mod tf_client;
mod widget;

use crate::model::tf_plan::TfPlan;

#[derive(Parser, Debug)]
#[command(version, about = "A TUI for working with OpenTofu and Terraform", long_about = None)]
struct Args {
    #[arg(short, long, help = "A path to a plan file (binary or JSON)")]
    plan_file: Option<PathBuf>,
    #[arg(
        short,
        long,
        default_value = "tofu",
        help = "The name/path of a TF binary"
    )]
    binary: String,
    #[arg(
        long,
        action,
        default_value_t = false,
        help = "Disable big red warning"
    )]
    hide_experimental_warning: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    let plan_file = args.plan_file;
    let show_experimental_warning = !args.hide_experimental_warning;

    let tf_client = TfClient::new(args.binary);

    let (diff, text_plan) = match plan_file {
        Some(p) => generate_diff_from_plan(p, &tf_client),
        None => {
            let (diff, plan) = generate_diff(&tf_client)?;
            Ok((diff, Some(plan)))
        }
    }?;

    let mut terminal = ratatui::init();
    let mut app = AppState::new(diff, text_plan, show_experimental_warning);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

type TextPlan = String;

fn generate_diff(client: &TfClient) -> Result<(TrowelDiff, TextPlan), io::Error> {
    // NOTE: NamedTempFile automatically deletes its tempfile when dropped via its destructor, and so should be dropped explicitly
    let binary_tempfile = client.plan()?;
    let (diff, plan) = generate_diff_binary(binary_tempfile.path().to_path_buf(), client)?;
    drop(binary_tempfile);
    Ok((diff, plan))
}

fn is_json_file(path: &Path) -> bool {
    let extension = path.extension().and_then(OsStr::to_str);
    matches!(extension, Some("json"))
}

fn generate_diff_from_plan(
    plan_file: PathBuf,
    client: &TfClient,
) -> Result<(TrowelDiff, Option<TextPlan>), io::Error> {
    if is_json_file(plan_file.as_path()) {
        let json_data = fs::read_to_string(&plan_file)?;
        Ok((generate_diff_json(json_data)?, None))
    } else {
        let (diff, plan) = generate_diff_binary(plan_file, client)?;
        Ok((diff, Some(plan)))
    }
}

fn generate_diff_json(json_data: String) -> Result<TrowelDiff, io::Error> {
    let parsed: TfPlan = serde_json::from_str(&json_data)?;
    TrowelDiff::from_tf_plan(&parsed)
}

fn generate_diff_binary(
    plan_file: PathBuf,
    tf_client: &TfClient,
) -> Result<(TrowelDiff, TextPlan), io::Error> {
    let text_plan = tf_client.show_as_text(&plan_file)?;
    let json_plan = tf_client.show_as_json(&plan_file)?;
    let diff = generate_diff_json(json_plan)?;
    Ok((diff, text_plan))
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    loop {
        match app.lifecycle {
            Lifecycle::Running => {
                terminal.draw(|f| ui(f, app))?;
                app.process_event(event::read()?);
            }
            Lifecycle::Quit => return Ok(()),
        }
    }
}

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let ui = AppView::new();
    frame.render_stateful_widget(ui, frame.area(), app);
}
