use tui_tree_widget::TreeState;

use crate::model::trowel_diff::TrowelDiff;

pub struct AppState {
    pub active_window: Window,
    pub tree_state: TreeState<String>,
    pub diff: TrowelDiff,
    pub text_plan_state: Option<TextPlanState>,
}

impl AppState {
    pub fn new(diff: TrowelDiff, text_plan: Option<String>) -> AppState {
        AppState {
            active_window: Window::TreeView,
            tree_state: TreeState::default(),
            diff,
            text_plan_state: text_plan.map(TextPlanState::new),
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

pub struct TextPlanState {
    pub plan: String,
    pub scroll_y: u16,
}

impl TextPlanState {
    pub fn new(plan: String) -> Self {
        Self {
            plan,
            scroll_y: 0
        }
    }

    /// Scrolls up by 1 row
    ///
    /// Returns true if scrolled, false if already at the top
    pub fn scroll_up(&mut self) -> bool {
        if self.scroll_y > 0 {
            self.scroll_y -= 1;
            true
        } else {
            false
        }
    }

    /// Scrolls down by 1 row
    ///
    /// Returns true if scrolled, false if already at the bottom
    pub fn scroll_down(&mut self) -> bool {
        self.scroll_y += 1;
        true
    }
}