use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

pub struct ErrorView {
    message: String,
}

impl Widget for ErrorView {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Self::outer_block().render(area, buf);
        self.render_popup(area, buf);
    }
}

impl ErrorView {
    pub fn new(message: String) -> Self {
        ErrorView { message }
    }

    fn title() -> Span<'static> {
        Span::styled(" Trowel ", Style::default().fg(Color::Blue)).add_modifier(Modifier::BOLD)
    }

    fn outer_block() -> impl Widget {
        Block::bordered().title(Self::title())
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        area
    }

    fn render_popup(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Thick)
            .style(Color::Yellow);
        let style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let text: Vec<Line> = self
            .message
            .lines()
            .map(|l| Span::styled(l, style))
            .map(Line::from)
            .collect();
        let popup = Paragraph::new(text).block(block).centered();

        let area = Self::center(
            area,
            Constraint::Length(
                self.message.lines().map(|l| l.len()).max().unwrap_or(94) as u16 + 6,
            ),
            Constraint::Length(self.message.lines().count() as u16 + 2),
        );

        Clear.render(area, buf);
        popup.render(area, buf);
    }
}
