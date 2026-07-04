use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

const TAB_KEYBINDINGS: &[&str] = &[
    "[1] Dash  [2] Explore  [3] Payload  [4] Sessions  [5] Console  [6] Scripts",
    "[?] Help  [q] Quit",
];

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, area: Rect, _tab: usize, message: &str) {
        let style = Style::default().add_modifier(Modifier::REVERSED);

        let hint = TAB_KEYBINDINGS[0];

        let text = if message.is_empty() {
            Line::from(Span::styled(format!(" {hint}"), style))
        } else {
            Line::from(vec![
                Span::styled(format!(" {hint}"), style),
                Span::raw("  "),
                Span::styled(
                    format!(" 〉 {}", message),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ])
        };

        f.render_widget(Paragraph::new(text).style(style), area);
    }
}
