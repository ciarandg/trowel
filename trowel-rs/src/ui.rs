use ratatui::{
    layout::{Constraint, Direction, Layout}, Frame
};

use crate::app::AppState;
use crate::widget::trowel_view::TrowelView;

pub fn ui(frame: &mut Frame, app: &mut AppState) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
        ])
        .split(frame.area());

    if let Some(chunk) = chunks.first() {
        let ui = TrowelView::new();
        frame.render_stateful_widget(ui, *chunk, app);
    } // TODO add error screen for when chunks.first() is None
}