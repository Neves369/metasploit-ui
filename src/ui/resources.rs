use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::msf::msfconsole;

#[derive(Clone)]
pub struct ScriptFile {
    pub name: String,
    pub path: String,
    pub size: String,
}

pub struct Resources {
    pub scripts: Vec<ScriptFile>,
    pub selected: Option<usize>,
    pub content: String,
    pub show_content: bool,
    pub feedback_message: String,
}

impl Resources {
    pub fn new() -> Self {
        let scripts = Self::scan_scripts();

        Self {
            scripts,
            selected: None,
            content: String::new(),
            show_content: false,
            feedback_message: String::new(),
        }
    }

    fn scan_scripts() -> Vec<ScriptFile> {
        let mut scripts = Vec::new();
        let dirs = [".", "./scripts"];

        for dir in &dirs {
            let path = Path::new(dir);
            if !path.is_dir() {
                continue;
            }
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().map_or(false, |e| e == "rc") {
                        let size = fs::metadata(&p)
                            .ok()
                            .map(|m| {
                                let s = m.len();
                                if s < 1024 {
                                    format!("{s} B")
                                } else {
                                    format!("{:.1} KB", s as f64 / 1024.0)
                                }
                            })
                            .unwrap_or_default();
                        scripts.push(ScriptFile {
                            name: p
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            path: p.to_string_lossy().to_string(),
                            size,
                        });
                    }
                }
            }
        }

        scripts.sort_by(|a, b| a.name.cmp(&b.name));
        scripts
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.feedback_message.clear();

        if self.show_content {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.show_content = false;
                    return true;
                }
                _ => return false,
            }
        }

        match key.code {
            KeyCode::Up => {
                self.selected = Some(self.selected.map_or(0, |i| i.saturating_sub(1)));
                true
            }
            KeyCode::Down => {
                if self.scripts.is_empty() {
                    return true;
                }
                let next = self
                    .selected
                    .map_or(0, |i| (i + 1).min(self.scripts.len() - 1));
                self.selected = Some(next);
                true
            }
            KeyCode::Enter => {
                if let Some(idx) = self.selected {
                    if idx < self.scripts.len() {
                        let script = &self.scripts[idx];
                        match fs::read_to_string(&script.path) {
                            Ok(text) => {
                                self.content = text;
                                self.show_content = true;
                            }
                            Err(e) => {
                                self.feedback_message =
                                    format!("Failed to read {}: {e}", script.name);
                            }
                        }
                    }
                }
                true
            }
            KeyCode::Char('r') => {
                if let Some(idx) = self.selected {
                    if idx < self.scripts.len() {
                        let script = &self.scripts[idx];
                        match msfconsole::run_resource_script(&script.path) {
                            Ok(output) => {
                                let lines: Vec<&str> =
                                    output.lines().filter(|l| !l.is_empty()).collect();
                                let preview = if lines.len() > 5 {
                                    format!("{} (...)", lines[..5].join(" | "))
                                } else {
                                    lines.join(" | ")
                                };
                                self.feedback_message =
                                    format!("[r] {}: {}", script.name, preview);
                            }
                            Err(e) => {
                                self.feedback_message =
                                    format!("[r] Error running {}: {e}", script.name);
                            }
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        _message: &str,
    ) -> String {
        if !self.feedback_message.is_empty() {
            return self.feedback_message.clone();
        }

        if self.show_content {
            return self.render_content_view(f, area);
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        self.render_file_list(f, chunks[0]);
        self.render_info_panel(f, chunks[1]);

        if let Some(idx) = self.selected {
            if idx < self.scripts.len() {
                return format!("Script: {}", self.scripts[idx].name);
            }
        }
        String::new()
    }

    fn render_file_list(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Resource Scripts ")
            .borders(Borders::ALL);

        if self.scripts.is_empty() {
            let msg = Line::from(Span::styled(
                " No .rc files found in . or ./scripts/",
                Style::default().add_modifier(Modifier::DIM),
            ));
            f.render_widget(Paragraph::new(vec![msg]).block(block), area);
            return;
        }

        let mut lines = vec![Line::from(vec![
            Span::styled(" Name                  ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Size    ", Style::default().add_modifier(Modifier::BOLD)),
        ])];

        for (i, script) in self.scripts.iter().enumerate() {
            let is_sel = self.selected == Some(i);
            let style = if is_sel {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            lines.push(Line::from(vec![
                Span::styled(format!(" {:<21}", script.name), style),
                Span::styled(format!(" {:7}", script.size), style),
            ]));
        }

        f.render_widget(Paragraph::new(lines).block(block), area);
    }

    fn render_info_panel(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Actions ")
            .borders(Borders::ALL);

        let lines = vec![
            Line::from(Span::styled(
                " [Enter] View content  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
            Line::from(Span::styled(
                " [r] Run script  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
            Line::from(Span::styled(
                " [↑/↓] Navigate  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::raw(
                "Resource scripts (.rc) contain\nMetasploit commands executed\nsequentially by msfconsole.",
            )),
        ];

        f.render_widget(Paragraph::new(lines).block(block), area);
    }

    fn render_content_view(&self, f: &mut Frame, area: Rect) -> String {
        let block = Block::default()
            .title(" Script Content ")
            .borders(Borders::ALL);

        let lines: Vec<Line> = self
            .content
            .lines()
            .map(|line| Line::from(Span::raw(line)))
            .collect();

        let hint = Line::from(Span::styled(
            " [Esc/Enter] Back to list  ",
            Style::default().add_modifier(Modifier::DIM),
        ));

        let mut all_lines = lines;
        all_lines.push(Line::from(Span::raw("")));
        all_lines.push(hint);

        f.render_widget(
            Paragraph::new(all_lines).block(block).wrap(Wrap { trim: false }),
            area,
        );

        String::new()
    }
}
