use tui_tree_widget::{TreeItem, TreeState};

pub struct App<'a> {
    pub state: TreeState<String>,
    pub items: Vec<TreeItem<'a, String>>,
}

impl App<'_> {
    pub fn new(tree_items: Vec<TreeItem<String>>) -> App {
        App {
            state: TreeState::default(),
            items: tree_items,
        }
    }
}