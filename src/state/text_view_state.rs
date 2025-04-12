use ratatui::crossterm::event::{KeyCode, KeyEvent};
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

    pub fn process_keypress(&mut self, key: &KeyEvent) {
        match key.code {
            // Basic navigation
            KeyCode::Char('j') => {
                self.scroll_view_state.scroll_down()
            },
            KeyCode::Char('k') => {
                self.scroll_view_state.scroll_up()
            },
            _ => ()
        }
    }
}