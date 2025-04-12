use ratatui::{
    layout::{Constraint, Direction, Layout}, Frame
};

use crate::{state::app_state::AppState, widget::app_view::AppView};

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
        ])
        .split(frame.area());

    if let Some(chunk) = chunks.first() {
        let ui = AppView::new();
        frame.render_stateful_widget(ui, *chunk, app);
    } // TODO add error screen for when chunks.first() is None
}