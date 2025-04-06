use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap}, Frame
};
use tui_tree_widget::Tree;

use crate::app::AppState;

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
        ])
        .split(frame.area());

    let title = Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD);

    if let Some(chunk) = chunks.first() {
        match app.active_window {
            crate::app::Window::TreeView => render_tree_view(chunk, frame, app, title),
            crate::app::Window::TextView => render_text_view(chunk, frame, app, title),
        }
    } // TODO add error screen for when chunks.first() is None
}

fn render_tree_view(chunk: &Rect, frame: &mut Frame, app: &mut AppState, title: Span) {
    let tree_items = match app.diff.to_tree_items() {
        Ok(ti) => ti,
        Err(_) => return, // TODO add error screen
    };
    let tree = match Tree::new(&tree_items) {
        Ok(t) => {
            t
                .block(
                    Block::bordered()
                        .title(title)
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

    frame.render_stateful_widget(tree, *chunk, &mut app.tree_state);
}

fn render_text_view(chunk: &Rect, frame: &mut Frame, app: &mut AppState, title: Span) {
    if let Some(p) = app.text_plan_state.as_ref() {
        let text: Vec<Line> = p.plan.lines().map(|l| Line::from(l.to_string())).collect();
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(title))
            .style(Style::new().white().on_black())
            .wrap(Wrap { trim: true })
            .scroll((p.scroll_y, 0));
        frame.render_widget(paragraph, *chunk);
    } // TODO add error screen
}