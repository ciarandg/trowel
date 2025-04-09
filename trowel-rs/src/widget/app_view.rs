use ratatui::{
    buffer::Buffer, layout::Rect, widgets::StatefulWidget
};

use crate::state::app_state::{ActiveView, AppState};

use super::{text_view::TextView, tree_view::TreeView};

pub struct AppView {
    text_view: TextView,
    tree_view: TreeView,
}

impl StatefulWidget for AppView {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match state.active_view {
            ActiveView::TreeView => {
                self.tree_view.render(area, buf, &mut state.tree_view_state);
            },
            ActiveView::TextView => {
                if let Some(view) = state.text_view_state.as_mut() {
                    self.text_view.render(area, buf, view);
                }
                // TODO error screen
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
}
