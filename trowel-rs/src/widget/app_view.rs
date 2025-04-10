use ratatui::{
    buffer::Buffer, layout::Rect, widgets::{StatefulWidget, Widget}
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
        match state.active_view {
            ActiveView::TreeView => {
                self.tree_view.render(area, buf, &mut state.tree_view_state);
            },
            ActiveView::TextView => {
                match state.text_view_state.as_mut() {
                    Some(view) => self.text_view.render(area, buf, view),
                    None => ErrorView::new("No text plan available!\nYou are likely viewing a JSON plan.".to_string()).render(area, buf),
                }
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
