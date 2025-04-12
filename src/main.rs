use std::{
    error::Error,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Parser, command};
use model::trowel_diff::TrowelDiff;
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    crossterm::event::{self},
};
use state::app_state::{AppState, Lifecycle};
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

    let (binary_tempfile, plan_file) = match plan_file {
        Some(f) => (None, f),
        None => {
            let tempfile = tf_client.plan()?;
            let plan = tempfile.path().to_path_buf();
            (Some(tempfile), plan)
        }
    };
    let diff = generate_diff(&tf_client, &plan_file)?;
    let text_plan = generate_text_plan(&tf_client, &plan_file)?;
    if let Some(tempfile) = binary_tempfile {
        // NamedTempFile automatically deletes its tempfile when dropped via its destructor, and so should be dropped explicitly
        drop(tempfile);
    };

    let mut terminal = ratatui::init();
    let mut app = AppState::new(diff, text_plan, show_experimental_warning);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

type TextPlan = String;

fn generate_diff(client: &TfClient, plan_file: &PathBuf) -> Result<TrowelDiff, io::Error> {
    let json_plan = if is_json_file(plan_file) {
        fs::read_to_string(plan_file)?
    } else {
        client.show_as_json(plan_file)?
    };
    let parsed: TfPlan = serde_json::from_str(&json_plan)?;
    let diff = TrowelDiff::from_tf_plan(&parsed)?;
    Ok(diff)
}

fn generate_text_plan(
    client: &TfClient,
    plan_file: &PathBuf,
) -> Result<Option<TextPlan>, io::Error> {
    let text_plan = if is_json_file(plan_file) {
        None
    } else {
        Some(client.show_as_text(plan_file)?)
    };
    Ok(text_plan)
}

fn is_json_file(path: &Path) -> bool {
    let extension = path.extension().and_then(OsStr::to_str);
    matches!(extension, Some("json"))
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
