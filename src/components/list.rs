use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List as RatatuiList, ListItem};
use ratatui::Frame;

pub struct ScrollableList {
    pub title: String,
    pub items: Vec<String>,
    pub selected: Option<usize>,
    pub scroll_offset: usize,
}

impl ScrollableList {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
            selected: None,
            scroll_offset: 0,
        }
    }

    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let idx = self.selected.map_or(0, |i| (i + 1).min(self.items.len() - 1));
        self.selected = Some(idx);
        self.ensure_visible(idx);
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let idx = self.selected.map_or(0, |i| i.saturating_sub(1));
        self.selected = Some(idx);
        self.ensure_visible(idx);
    }

    fn ensure_visible(&mut self, idx: usize) {
        let height = 10usize;
        if idx < self.scroll_offset {
            self.scroll_offset = idx;
        } else if idx >= self.scroll_offset + height {
            self.scroll_offset = idx.saturating_sub(height).saturating_add(1);
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = self.selected == Some(i);
                let style = if is_selected {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(item.as_str()).style(style)
            })
            .collect();

        let list = RatatuiList::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_widget(list, area);
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = None;
        self.scroll_offset = 0;
    }
}
