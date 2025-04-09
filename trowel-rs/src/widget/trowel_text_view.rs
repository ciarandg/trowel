use ratatui::{
    buffer::Buffer, layout::{Rect, Size}, style::{Color, Modifier, Style, Stylize}, text::Span, widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap}
};
use tui_scrollview::ScrollView;

use crate::app::TextViewState;

const SCROLLVIEW_HEIGHT: u16 = 100; // TODO this value should depend on the length of the text

pub struct TrowelTextView;

impl StatefulWidget for TrowelTextView {
    type State = TextViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let width = if buf.area.height < SCROLLVIEW_HEIGHT {
            buf.area.width - 1
        } else {
            buf.area.width
        };
        let mut scroll_view = ScrollView::new(Size::new(width, SCROLLVIEW_HEIGHT));
        let text_plan = &state.plan;
        self.render_widgets_into_scrollview(scroll_view.buf_mut(), text_plan);
        scroll_view.render(area, buf, &mut state.scroll_view_state)
    }
}

impl TrowelTextView {
    pub fn new() -> Self {
        TrowelTextView {}
    }

    fn title() -> Span<'static> {
        Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD)
    }

    fn render_widgets_into_scrollview(&self, buf: &mut Buffer, text_plan: &str) {
        let area = buf.area;
        self.text(text_plan).render(area, buf);
    }

    fn text(&self, text: &str) -> impl Widget {
        let block = Block::bordered().title(Self::title());
        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .block(block)
    }
}
