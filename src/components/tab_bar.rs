use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

const TAB_TITLES: &[&str] = &[
    " 1  Dashboard ",
    " 2  Explorer  ",
    " 3  Payload   ",
    " 4  Sessions  ",
    " 5  Console   ",
    " 6  Scripts   ",
];

pub struct TabBar;

impl TabBar {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect, active: usize) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let spans: Vec<Span> = TAB_TITLES
            .iter()
            .enumerate()
            .flat_map(|(i, title)| {
                let style = if i == active {
                    Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::DIM)
                };
                let span = Span::styled(*title, style);
                let sep = Span::raw(" ");
                vec![span, sep]
            })
            .collect();

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line)
            .block(block)
            .style(Style::default());

        f.render_widget(paragraph, layout[0]);
    }
}
