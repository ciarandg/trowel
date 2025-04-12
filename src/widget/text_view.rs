use ratatui::{
    buffer::Buffer, layout::{Rect, Size}, style::{Color, Modifier, Style, Stylize}, text::Span, widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap}
};
use tui_scrollview::ScrollView;

use crate::state::text_view_state::TextViewState;

pub struct TextView;

impl StatefulWidget for TextView {
    type State = TextViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text_plan = &state.plan;

        // TODO figure out why 2 is the correct value. Also this breaks when text wraps
        let scrollview_height = text_plan.lines().count() as u16 + 2;

        let width = if buf.area.height < scrollview_height {
            buf.area.width - 1
        } else {
            buf.area.width
        };
        let mut scroll_view = ScrollView::new(Size::new(width, scrollview_height));
        self.render_widgets_into_scrollview(scroll_view.buf_mut(), text_plan);
        scroll_view.render(area, buf, &mut state.scroll_view_state)
    }
}

impl TextView {
    pub fn new() -> Self {
        TextView {}
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
