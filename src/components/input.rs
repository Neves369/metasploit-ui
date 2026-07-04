use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct InputField {
    pub label: String,
    pub value: String,
    pub cursor_pos: usize,
    pub focused: bool,
}

impl InputField {
    pub fn new(label: &str, default: &str) -> Self {
        let value = default.to_string();
        Self {
            label: label.to_string(),
            cursor_pos: value.len(),
            value,
            focused: false,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let title = if self.focused {
            format!(" ▌{} ", self.label)
        } else {
            format!(" {} ", self.label)
        };

        let border_style = if self.focused {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(self.value.as_str())
            .block(block)
            .style(Style::default());

        f.render_widget(paragraph, area);

        if self.focused {
            let x = area.x + 1 + self.cursor_pos as u16;
            let y = area.y + 1;
            if x < area.right() && y < area.bottom() {
                f.set_cursor_position((x, y));
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.value.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_pos = self.value.len();
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_pos = 0;
    }
}
