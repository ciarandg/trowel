use std::{
    error::Error,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Parser, command};
use model::trowel_diff::TrowelDiff;
use ratatui::{Frame, Terminal, backend::Backend, crossterm::event};
use state::{
    app_state::{AppState, Lifecycle},
    planning_view_state::PlanningViewState,
};
use tf_client::TfClient;
use tokio::task::JoinHandle;
use widget::{app_view::AppView, planning_view::PlanningView};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    let plan_file = args.plan_file;
    let show_experimental_warning = !args.hide_experimental_warning;

    let tf_client = TfClient::new(args.binary);

    // Initialization is deferred to avoid delay between init and first paint
    let mut terminal: Option<Terminal<_>> = None;

    let (binary_tempfile, plan_file) = match plan_file {
        Some(f) => (None, f),
        None => {
            let (mut planning_view_state, tx) = PlanningViewState::new();
            let tf_client = tf_client.clone();
            let handle = tokio::spawn(async move { tf_client.plan(tx).await });
            let mut term = terminal.unwrap_or_else(ratatui::init);
            run_app_preinit(&mut term, &mut planning_view_state, &handle).await?;
            terminal = Some(term);
            let tempfile = handle.await??;
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

    let mut terminal = terminal.unwrap_or_else(ratatui::init);
    let mut app = AppState::new(diff, text_plan, show_experimental_warning);
    run_app(&mut terminal, &mut app).await?;
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

async fn run_app_preinit<B: Backend, T>(
    terminal: &mut Terminal<B>,
    planning_view_state: &mut PlanningViewState,
    handle: &JoinHandle<T>,
) -> io::Result<()> {
    // Painting immediately prevents blank screen
    terminal.draw(|f| ui_preinit(f, planning_view_state))?;

    loop {
        tokio::select! {
            Some(()) = planning_view_state.next_line() => {
                terminal.draw(|f| ui_preinit(f, planning_view_state))?;
            }
            else => {
                if handle.is_finished() {
                    return Ok(())
                }
            }
        }
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
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

fn ui_preinit(frame: &mut Frame, state: &mut PlanningViewState) {
    let ui = PlanningView::new();
    frame.render_stateful_widget(ui, frame.area(), state);
}

fn ui(frame: &mut Frame, state: &mut AppState) {
    let ui = AppView::new();
    frame.render_stateful_widget(ui, frame.area(), state);
}
