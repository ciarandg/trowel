use tui_tree_widget::TreeState;

use crate::model::trowel_diff::TrowelDiff;

pub struct App {
    pub state: TreeState<String>,
    pub diff: TrowelDiff,
}

impl App {
    pub fn new(diff: TrowelDiff) -> App {
        App {
            state: TreeState::default(),
            diff,
        }
    }
}