use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::state::app_state::{ActiveView, AppState};

use super::{error_view::ErrorView, text_view::TextView, tree_view::TreeView};

pub struct AppView {
    text_view: TextView,
    tree_view: TreeView,
}

impl StatefulWidget for AppView {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (a1, a2) = Self::experimental_warning_layout(area);
        let area = if state.show_experimental_warning {
            a2
        } else {
            area
        };
        if state.show_experimental_warning {
            Self::experimental_warning().render(a1, buf);
        }

        match state.active_view {
            ActiveView::TreeView => {
                self.tree_view.render(area, buf, &mut state.tree_view_state);
            }
            ActiveView::TextView => match state.text_view_state.as_mut() {
                Some(view) => self.text_view.render(area, buf, view),
                None => ErrorView::new(
                    "No text plan available!\nYou are likely viewing a JSON plan.".to_string(),
                    Color::Yellow,
                )
                .render(area, buf),
            },
        }
    }
}

impl AppView {
    pub fn new() -> Self {
        Self {
            text_view: TextView::new(),
            tree_view: TreeView::new(),
        }
    }

    fn experimental_warning() -> impl Widget {
        let block = Block::bordered()
            .border_type(BorderType::Thick)
            .style(Color::Red);
        let style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
        let text = Line::from(Span::styled(
            "CAUTION: This app is experimental and untested. Do not trust its output!",
            style,
        ));
        Paragraph::new(text).block(block).centered()
    }

    fn experimental_warning_layout(area: Rect) -> (Rect, Rect) {
        let [a1, a2] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
            .areas(area);
        (a1, a2)
    }
}
