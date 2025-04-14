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
        const PAGE_DOWN_LINES: u16 = 10;

        match key.code {
            // Basic navigation
            KeyCode::Char('j') => self.scroll_view_state.scroll_down(),
            KeyCode::Char('k') => self.scroll_view_state.scroll_up(),
            KeyCode::Char('u') => {
                for _ in 0..PAGE_DOWN_LINES {
                    self.scroll_view_state.scroll_up();
                }
            }
            KeyCode::Char('d') => {
                for _ in 0..PAGE_DOWN_LINES {
                    self.scroll_view_state.scroll_down();
                }
            }
            _ => (),
        }
    }
}
