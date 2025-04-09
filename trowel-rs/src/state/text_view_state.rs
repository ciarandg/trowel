use tui_scrollview::ScrollViewState;

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