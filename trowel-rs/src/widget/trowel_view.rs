use ratatui::{
    buffer::Buffer, layout::Rect, widgets::StatefulWidget
};

use crate::app::AppState;

use super::{trowel_text_view::TrowelTextView, trowel_tree_view::TrowelTreeView};

pub struct TrowelView {
    text_view: TrowelTextView,
    tree_view: TrowelTreeView,
}

impl StatefulWidget for TrowelView {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match state.active_window {
            crate::app::Window::TreeView => {
                self.tree_view.render(area, buf, &mut state.tree_view_state);
            },
            crate::app::Window::TextView => {
                if let Some(view) = state.text_view_state.as_mut() {
                    self.text_view.render(area, buf, view);
                }
                // TODO error screen
            },
        }
    }
}

impl TrowelView {
    pub fn new() -> Self {
        Self {
            text_view: TrowelTextView::new(),
            tree_view: TrowelTreeView::new(),
        }
    }
}
