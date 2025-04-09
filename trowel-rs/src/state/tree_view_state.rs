use tui_tree_widget::TreeState;

use crate::model::trowel_diff::TrowelDiff;

pub struct TreeViewState {
    pub diff: TrowelDiff,
    pub tree_state: TreeState<String>,
}

impl TreeViewState {
    pub fn new(diff: TrowelDiff) -> Self {
        TreeViewState {
            diff,
            tree_state: TreeState::default(),
        }
    }
}