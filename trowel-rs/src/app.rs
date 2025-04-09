use tui_scrollview::ScrollViewState;
use tui_tree_widget::TreeState;

use crate::model::trowel_diff::TrowelDiff;

pub struct AppState {
    pub active_window: Window,
    pub text_view_state: Option<TextViewState>,
    pub tree_view_state: TreeViewState,
}

impl AppState {
    pub fn new(diff: TrowelDiff, text_plan: Option<String>) -> AppState {
        AppState {
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

#[derive(PartialEq)]
pub enum Window {
    TreeView,
    TextView,
}

pub struct TextViewState {
    pub plan: String,
    pub scroll_view_state: ScrollViewState,
}

impl TextViewState {
    pub fn new(plan: String) -> Self {
        Self {
            plan,
            scroll_view_state: ScrollViewState::new(),
        }
    }
}

pub struct TreeViewState {
    pub diff: TrowelDiff,
    pub tree_state: TreeState<String>,
}

impl TreeViewState {
    fn new(diff: TrowelDiff) -> Self {
        TreeViewState {
            diff,
            tree_state: TreeState::default(),
        }
    }
}