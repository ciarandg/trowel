use std::{error::Error, io, fs, env};

use model::diff::{diff_from_tf_plan, tree_items_from_diff};
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_json_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let parsed: TfPlan = serde_json::from_str(&contents)?;
    let diff = diff_from_tf_plan(&parsed);
    let tree_items = tree_items_from_diff(&diff);

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new(tree_items);
    run_app(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

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