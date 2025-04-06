use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, text::Span, widgets::{Block, Scrollbar, ScrollbarOrientation}, Frame
};
use tui_tree_widget::Tree;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
        ])
        .split(frame.area());

        let tree_items = match app.diff.to_tree_items() {
            Ok(ti) => ti,
            Err(_) => return, // TODO add error screen
        };
        let tree = match Tree::new(&tree_items) {
            Ok(t) => {
                t
                    .block(
                        Block::bordered()
                            .title(Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD))
                            .title_bottom(app.diff.verb_uses_fmt()),
                    )
                    .experimental_scrollbar(Some(
                        Scrollbar::new(ScrollbarOrientation::VerticalRight)
                            .begin_symbol(None)
                            .track_symbol(None)
                            .end_symbol(None),
                    ))
                    .highlight_style(
                        Style::new()
                            .fg(Color::Black)
                            .bg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    )
            },
            Err(_) => {
                return // TODO add error screen
            },
        };

    if let Some(chunk) = chunks.first() {
        frame.render_stateful_widget(tree, *chunk, &mut app.state);
    } // TODO add error screen for when chunks.first() is None
}