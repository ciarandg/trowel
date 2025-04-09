use crate::model::trowel_diff::TrowelDiff;

use super::{text_view_state::TextViewState, tree_view_state::TreeViewState};

pub enum ActiveView {
    TreeView,
    TextView,
}

pub struct AppState {
    pub active_view: ActiveView,
    pub text_view_state: Option<TextViewState>,
    pub tree_view_state: TreeViewState,
}

impl AppState {
    pub fn new(diff: TrowelDiff, text_plan: Option<String>) -> AppState {
        AppState {
            active_view: ActiveView::TreeView,
            text_view_state: text_plan.map(TextViewState::new),
            tree_view_state: TreeViewState::new(diff),
        }
    }

    // Toggles between tree view and text view
    //
    // Returns `true` when switched
    pub fn toggle_view(&mut self) -> bool {
        self.active_view = match self.active_view {
            ActiveView::TreeView => ActiveView::TextView,
            ActiveView::TextView => ActiveView::TreeView,
        };
        true
    }
}