use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::model::trowel_diff::TrowelDiff;

use super::{text_view_state::TextViewState, tree_view_state::TreeViewState};

pub enum Lifecycle {
    Running,
    Quit,
}

pub enum ActiveView {
    TreeView,
    TextView,
}

pub struct AppState {
    pub lifecycle: Lifecycle,
    pub active_view: ActiveView,
    pub text_view_state: Option<TextViewState>,
    pub tree_view_state: TreeViewState,
}

impl AppState {
    pub fn new(diff: TrowelDiff, text_plan: Option<String>) -> AppState {
        AppState {
            lifecycle: Lifecycle::Running,
            active_view: ActiveView::TreeView,
            text_view_state: text_plan.map(TextViewState::new),
            tree_view_state: TreeViewState::new(diff),
        }
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => (),
            Event::Key(key) if Self::is_quit_binding(&key) => self.quit(),
            Event::Key(key) if key.code == KeyCode::Tab => self.toggle_view(),
            Event::Key(key) => match self.active_view {
                ActiveView::TreeView => {
                    self.tree_view_state.process_keypress(&key);
                },
                ActiveView::TextView => {
                    if let Some(state) = self.text_view_state.as_mut() {
                        state.process_keypress(&key);
                    }
                },
            },
            Event::Mouse(mouse) => match self.active_view {
                ActiveView::TreeView => self.tree_view_state.process_mouse_event(mouse),
                ActiveView::TextView => (),
            },
            Event::Resize(_, _) => (),
            _ => (),
        }
    }

    fn toggle_view(&mut self) {
        self.active_view = match self.active_view {
            ActiveView::TreeView => ActiveView::TextView,
            ActiveView::TextView => ActiveView::TreeView,
        }
    }

    fn quit(&mut self) {
        self.lifecycle = Lifecycle::Quit;
    }

    fn is_quit_binding(key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
            KeyCode::Char('q') => true,
            _ => false,
        }
    }
}