use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Modifier, Style, Stylize}, text::Span, widgets::{Block, Scrollbar, ScrollbarOrientation, StatefulWidget}
};
use tui_tree_widget::Tree;

use crate::{app::TreeViewState, model::trowel_diff::TrowelDiff};

pub struct TrowelTreeView {}

impl StatefulWidget for TrowelTreeView {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Ok(tree_items) = state.diff.to_tree_items() {
            if let Ok(t) = Tree::new(&tree_items) {
                let style = Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD);
                let tree = t.block(Self::wrapper_block(&state.diff))
                    .experimental_scrollbar(Some(Self::scrollbar()))
                    .highlight_style(style);
                tree.render(area, buf, &mut state.tree_state);
            };
        }
        // TODO error screen
    }
}

impl TrowelTreeView {
    pub fn new() -> Self {
        TrowelTreeView {}
    }

    fn title() -> Span<'static> {
        Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD)
    }

    fn wrapper_block(diff: &TrowelDiff) -> Block<'_> {
        Block::bordered()
            .title(Self::title())
            .title_bottom(diff.verb_uses_fmt())
    }

    fn scrollbar() -> Scrollbar<'static> {
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None)
    }
}