use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    layout::Position,
};
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

    pub fn process_keypress(&mut self, key: &KeyEvent) {
        match key.code {
            // Fold and unfold
            KeyCode::Enter => self.tree_state.toggle_selected(),

            // Basic navigation
            KeyCode::Char('h') => self.tree_state.key_left(),
            KeyCode::Char('l') => self.tree_state.key_right(),
            KeyCode::Char('j') => self.tree_state.key_down(),
            KeyCode::Char('k') => self.tree_state.key_up(),
            KeyCode::Left => self.tree_state.key_left(),
            KeyCode::Right => self.tree_state.key_right(),
            KeyCode::Down => self.tree_state.key_down(),
            KeyCode::Up => self.tree_state.key_up(),

            // Jump to top and bottom
            KeyCode::Char('g') => self.tree_state.select_first(),
            KeyCode::Char('G') => self.tree_state.select_last(),
            KeyCode::Home => self.tree_state.select_first(),
            KeyCode::End => self.tree_state.select_last(),
            _ => false,
        };
    }

    pub fn process_mouse_event(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => self.tree_state.scroll_down(1),
            MouseEventKind::ScrollUp => self.tree_state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .tree_state
                .click_at(Position::new(mouse.column, mouse.row)),
            _ => false,
        };
    }
}
