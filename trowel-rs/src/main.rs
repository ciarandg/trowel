use std::{error::Error, ffi::OsStr, fs, io, path::PathBuf, process::Command};

use clap::{command, Parser};
use model::trowel_diff::TrowelDiff;
use ratatui::{
    backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind}, layout::Position, Terminal
};

mod app;
mod ui;
mod model;

use crate::{
    app::App,
    ui::ui,
    model::tf_plan::TfPlan,
};

#[derive(Parser, Debug)]
#[command(version, about = "A TUI for working with OpenTofu and Terraform", long_about = None)]
struct Args {
    #[arg(short, long)]
    plan_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    let plan_file = args.plan_file;
    let diff = generate_diff(plan_file)?;

    let mut terminal = ratatui::init();
    let mut app = App::new(diff);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn generate_diff(plan_file: PathBuf) -> Result<TrowelDiff, io::Error> {
    let extension = plan_file.as_path().extension().and_then(OsStr::to_str);

    match extension {
        Some("json") => generate_diff_json(fs::read_to_string(plan_file)?),
        _ => generate_diff_binary(plan_file), // Assume binary plan if file extension is not '.json'
    }
}

fn generate_diff_json(json_data: String) -> Result<TrowelDiff, io::Error> {
    let parsed: TfPlan = serde_json::from_str(&json_data)?;
    TrowelDiff::from_tf_plan(&parsed)
}

fn generate_diff_binary(plan_file: PathBuf) -> Result<TrowelDiff, io::Error> {
    let output = Command::new("tofu")
        .arg("show")
        .arg("-json")
        .arg(plan_file)
        .output()?;
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    generate_diff_json(stdout.to_string())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app).unwrap())?;

        match event::read()? {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => false,
            Event::Key(key) => match key.code {
                // Exit program
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(())
                }
                KeyCode::Char('q') => return Result::Ok(()),

                // Fold and unfold
                KeyCode::Enter => app.state.toggle_selected(),

                // Basic navigation
                KeyCode::Char('h') => app.state.key_left(),
                KeyCode::Char('l') => app.state.key_right(),
                KeyCode::Char('j') => app.state.key_down(),
                KeyCode::Char('k') => app.state.key_up(),
                KeyCode::Left => app.state.key_left(),
                KeyCode::Right => app.state.key_right(),
                KeyCode::Down => app.state.key_down(),
                KeyCode::Up => app.state.key_up(),

                // Jump to top and bottom
                KeyCode::Char('g') => app.state.select_first(),
                KeyCode::Char('G') => app.state.select_last(),
                KeyCode::Home => app.state.select_first(),
                KeyCode::End => app.state.select_last(),
                _ => false,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => app.state.scroll_down(1),
                MouseEventKind::ScrollUp => app.state.scroll_up(1),
                MouseEventKind::Down(_button) => {
                    app.state.click_at(Position::new(mouse.column, mouse.row))
                }
                _ => false,
            },
            Event::Resize(_, _) => true,
            _ => false,
        };
    }
}