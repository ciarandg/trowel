use std::{error::Error, ffi::OsStr, fs, io, path::{Path, PathBuf}, process::{Command, Stdio}};

use clap::{command, Parser};
use model::trowel_diff::TrowelDiff;
use ratatui::{
    backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEventKind}, layout::Position, Terminal
};
use tempfile::NamedTempFile;

mod app;
mod ui;
mod model;

use crate::{
    app::AppState,
    ui::ui,
    model::tf_plan::TfPlan,
};

#[derive(Parser, Debug)]
#[command(version, about = "A TUI for working with OpenTofu and Terraform", long_about = None)]
struct Args {
    #[arg(short, long)]
    plan_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    let plan_file = args.plan_file;
    let (diff, text_plan) = match plan_file {
        Some(p) => generate_diff_from_plan(p),
        None => {
            let (diff, plan) = generate_diff()?;
            Ok((diff, Some(plan)))
        }
    }?;

    let mut terminal = ratatui::init();
    let mut app = AppState::new(diff, text_plan);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn generate_binary_plan() -> Result<NamedTempFile, io::Error> {
    let file = NamedTempFile::new()?;
    match file.path().to_str() {
        Some(p) => {
            let mut cmd = Command::new("tofu")
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

fn generate_diff() -> Result<(TrowelDiff, TextPlan), io::Error> {
    // NOTE: NamedTempFile automatically deletes its tempfile when dropped via its destructor, and so should be dropped explicitly
    let binary_tempfile = generate_binary_plan()?;
    let (diff, plan) = generate_diff_binary(binary_tempfile.path().to_path_buf())?;
    drop(binary_tempfile);
    Ok((diff, plan))
}

fn is_json_file(path: &Path) -> bool {
    let extension = path.extension().and_then(OsStr::to_str);
    matches!(extension, Some("json"))
}

fn generate_diff_from_plan(plan_file: PathBuf) -> Result<(TrowelDiff, Option<TextPlan>), io::Error> {
    if is_json_file(plan_file.as_path()) {
        let json_data = fs::read_to_string(&plan_file)?;
        Ok((generate_diff_json(json_data)?, None))
    } else {
        let (diff, plan) = generate_diff_binary(plan_file)?;
        Ok((diff, Some(plan)))
    }
}

fn generate_diff_json(json_data: String) -> Result<TrowelDiff, io::Error> {
    let parsed: TfPlan = serde_json::from_str(&json_data)?;
    TrowelDiff::from_tf_plan(&parsed)
}

fn generate_diff_binary(plan_file: PathBuf) -> Result<(TrowelDiff, TextPlan), io::Error> {
    let text_plan = generate_text_plan(&plan_file)?;
    let output = Command::new("tofu")
        .arg("show")
        .arg("-json")
        .arg(plan_file)
        .output()?;
    match std::str::from_utf8(&output.stdout) {
        Ok(out) => Ok((generate_diff_json(out.to_string())?, text_plan)),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse JSON plan stdout into UTF-8")),
    }
}

fn generate_text_plan(binary_plan: &PathBuf) -> Result<TextPlan, io::Error> {
    let output = Command::new("tofu")
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
        terminal.draw(|f| ui(f, app))?;

        match event::read()? {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => false,
            Event::Key(key) if is_quit_binding(&key) => return Ok(()),
            Event::Key(key) => match app.active_window {
                app::Window::TreeView => tree_view_binding(app, &key),
                app::Window::TextView => text_view_binding(app, &key),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => app.tree_state.scroll_down(1),
                MouseEventKind::ScrollUp => app.tree_state.scroll_up(1),
                MouseEventKind::Down(_button) => {
                    app.tree_state.click_at(Position::new(mouse.column, mouse.row))
                }
                _ => false,
            },
            Event::Resize(_, _) => true,
            _ => false,
        };
    }
}

/// Returns true if KeyEvent represents a quit bindings (`q` or Ctrl+c), else false
fn is_quit_binding(key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
        KeyCode::Char('q') => true,
        _ => false,
    }
}

fn tree_view_binding(app: &mut AppState, key: &KeyEvent) -> bool {
    match key.code {
        // Switch between views
        KeyCode::Tab => app.toggle_view(),

        // Fold and unfold
        KeyCode::Enter => app.tree_state.toggle_selected(),

        // Basic navigation
        KeyCode::Char('h') => app.tree_state.key_left(),
        KeyCode::Char('l') => app.tree_state.key_right(),
        KeyCode::Char('j') => app.tree_state.key_down(),
        KeyCode::Char('k') => app.tree_state.key_up(),
        KeyCode::Left => app.tree_state.key_left(),
        KeyCode::Right => app.tree_state.key_right(),
        KeyCode::Down => app.tree_state.key_down(),
        KeyCode::Up => app.tree_state.key_up(),

        // Jump to top and bottom
        KeyCode::Char('g') => app.tree_state.select_first(),
        KeyCode::Char('G') => app.tree_state.select_last(),
        KeyCode::Home => app.tree_state.select_first(),
        KeyCode::End => app.tree_state.select_last(),
        _ => false,
    }
}

fn text_view_binding(app: &mut AppState, key: &KeyEvent) -> bool {
    match key.code { // TODO deduplicate bindings with TextView
        // Switch between views
        KeyCode::Tab => app.toggle_view(),

        // Basic navigation
        KeyCode::Char('j') => {
            let state = app.text_plan_state.as_mut();
            match state {
                Some(s) => s.scroll_down(),
                None => false,
            }
        },
        KeyCode::Char('k') => {
            let state = app.text_plan_state.as_mut();
            match state {
                Some(s) => s.scroll_up(),
                None => false,
            }
        },
        _ => false
    }
}