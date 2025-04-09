use crate::{app::Window, model::trowel_diff::TrowelDiff};

use super::{trowel_text_view_state::TextViewState, trowel_tree_view_state::TreeViewState};

pub struct TrowelState {
    pub active_window: Window,
    pub text_view_state: Option<TextViewState>,
    pub tree_view_state: TreeViewState,
}

impl TrowelState {
    pub fn new(diff: TrowelDiff, text_plan: Option<String>) -> TrowelState {
        TrowelState {
            active_window: Window::TreeView,
            text_view_state: text_plan.map(TextViewState::new),
            tree_view_state: TreeViewState::new(diff),
        }
    }

    // Toggles between tree view and text view
    //
    // Returns `true` when switched
    pub fn toggle_view(&mut self) -> bool {
        self.active_window = match self.active_window {
            Window::TreeView => Window::TextView,
            Window::TextView => Window::TreeView,
        };
        true
    }
}