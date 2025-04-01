use std::io;
use std::result::Result;

use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Scrollbar, ScrollbarOrientation}, Frame
};
use tui_tree_widget::Tree;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) -> Result<(), io::Error> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
        ])
        .split(frame.area());

        let tree = Tree::new(&app.items)?
            .block(
                Block::bordered()
                    .title(Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD))
                    .title_bottom(Line::from(vec![
                        Span::styled(" create 0", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::from(" | "),
                        Span::styled("destroy 0", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                        Span::from(" | "),
                        Span::styled("replace 0", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                        Span::from(" | "),
                        Span::styled("update 0 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    ])),
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
            );

    frame.render_stateful_widget(tree, chunks[0], &mut app.state);
    Ok(())
}