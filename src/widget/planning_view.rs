use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::state::planning_view_state::PlanningViewState;

pub struct PlanningView {}

impl StatefulWidget for PlanningView {
    type State = PlanningViewState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let lines: Vec<Line<'_>> = state
            .plan_stdout
            .iter()
            .map(|l| Span::styled(l, Style::default().fg(Color::Gray)))
            .map(Line::from)
            .collect();
        let block = Block::bordered().title(Self::title());
        let line_count = lines.len();
        let paragraph = Paragraph::new(lines)
            .block(block)
            .scroll((Self::scroll_y(line_count, &area), 0));
        paragraph.render(area, buf);
    }
}

impl PlanningView {
    const BLOCK_HEIGHT: u16 = 2;

    pub fn new() -> Self {
        Self {}
    }

    fn title() -> Span<'static> {
        Span::styled(" Initializing... ", Style::default().fg(Color::Yellow))
            .add_modifier(Modifier::BOLD)
    }

    fn scroll_y(line_count: usize, area: &Rect) -> u16 {
        let line_count = line_count as u16;
        let viewport_height = area.height.saturating_sub(Self::BLOCK_HEIGHT);
        line_count.saturating_sub(viewport_height)
    }
}
