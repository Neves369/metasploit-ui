use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::components::input::InputField;

#[derive(Clone)]
pub struct MsfModule {
    pub name: String,
    pub rank: String,
    pub description: String,
    pub disclosure: Option<String>,
}

pub struct Explorer {
    pub categories: Vec<String>,
    pub selected_category: Option<usize>,
    pub modules: Vec<MsfModule>,
    pub selected_module: Option<usize>,
    pub search_input: InputField,
    pub show_detail: bool,
}

impl Explorer {
    pub fn new() -> Self {
        let categories = vec![
            "All".to_string(),
            "exploit".to_string(),
            "auxiliary".to_string(),
            "payload".to_string(),
            "post".to_string(),
            "encoder".to_string(),
            "nop".to_string(),
            "evasion".to_string(),
        ];

        let modules = vec![
            MsfModule {
                name: "exploit/multi/handler".into(),
                rank: "excellent".into(),
                description: "Generic handler for exploits".into(),
                disclosure: Some("2011-01-01".into()),
            },
            MsfModule {
                name: "exploit/multi/script/web_delivery".into(),
                rank: "manual".into(),
                description: "Script web delivery".into(),
                disclosure: None,
            },
            MsfModule {
                name: "exploit/multi/http/tomcat_mgr_upload".into(),
                rank: "excellent".into(),
                description: "Tomcat manager upload".into(),
                disclosure: Some("2009-11-09".into()),
            },
            MsfModule {
                name: "payload/windows/x64/meterpreter/reverse_tcp".into(),
                rank: "excellent".into(),
                description: "Meterpreter reverse TCP".into(),
                disclosure: None,
            },
            MsfModule {
                name: "auxiliary/scanner/portscan/tcp".into(),
                rank: "normal".into(),
                description: "TCP port scanner".into(),
                disclosure: None,
            },
            MsfModule {
                name: "post/linux/gather/hashdump".into(),
                rank: "normal".into(),
                description: "Linux hash dump".into(),
                disclosure: None,
            },
        ];

        Self {
            categories,
            selected_category: Some(0),
            modules,
            selected_module: None,
            search_input: InputField::new("Search", ""),
            show_detail: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.search_input.focused {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.search_input.focused = false;
                    return true;
                }
                KeyCode::Char(c) => {
                    self.search_input.insert_char(c);
                    return true;
                }
                KeyCode::Backspace => {
                    self.search_input.delete_char();
                    return true;
                }
                KeyCode::Delete => {
                    self.search_input.delete_forward();
                    return true;
                }
                KeyCode::Left => {
                    self.search_input.move_left();
                    return true;
                }
                KeyCode::Right => {
                    self.search_input.move_right();
                    return true;
                }
                KeyCode::Home => {
                    self.search_input.move_home();
                    return true;
                }
                KeyCode::End => {
                    self.search_input.move_end();
                    return true;
                }
                _ => return false,
            }
        }

        match key.code {
            KeyCode::Char('/') => {
                self.search_input.focused = true;
                self.search_input.clear();
                true
            }
            KeyCode::Up => {
                if self.show_detail {
                    self.show_detail = false;
                } else {
                    let prev = self.selected_category.unwrap_or(0);
                    self.selected_category = Some(prev.saturating_sub(1));
                }
                true
            }
            KeyCode::Down => {
                if self.show_detail {
                    // stay in detail
                } else {
                    let next = self.selected_category.unwrap_or(0) + 1;
                    if next < self.categories.len() {
                        self.selected_category = Some(next);
                    }
                }
                true
            }
            KeyCode::Right => {
                if self.selected_module.is_none() && !self.modules.is_empty() {
                    self.selected_module = Some(0);
                } else if let Some(idx) = self.selected_module {
                    if idx + 1 < self.modules.len() {
                        self.selected_module = Some(idx + 1);
                    }
                }
                true
            }
            KeyCode::Left => {
                if self.selected_module.is_some() {
                    if let Some(idx) = self.selected_module {
                        if idx > 0 {
                            self.selected_module = Some(idx - 1);
                        } else {
                            self.selected_module = None;
                        }
                    }
                }
                true
            }
            KeyCode::Enter => {
                if self.selected_module.is_some() {
                    self.show_detail = !self.show_detail;
                }
                true
            }
            KeyCode::Esc => {
                if self.show_detail {
                    self.show_detail = false;
                    true
                } else {
                    false
                }
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_search_bar(f, chunks[0]);

        if self.show_detail && self.selected_module.is_some() {
            let idx = self.selected_module.unwrap();
            if idx < self.modules.len() {
                self.render_module_detail(f, chunks[1], &self.modules[idx]);
                return format!("Viewing: {}", self.modules[idx].name);
            }
        }

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(chunks[1]);

        self.render_category_list(f, main_chunks[0]);
        self.render_module_list(f, main_chunks[1]);

        if let Some(idx) = self.selected_module {
            if idx < self.modules.len() {
                return format!("Selected: {}", self.modules[idx].name);
            }
        }

        String::new()
    }

    fn render_search_bar(&self, f: &mut Frame, area: Rect) {
        self.search_input.render(f, area);
    }

    fn render_category_list(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Categories ")
            .borders(Borders::ALL);

        let items: Vec<ratatui::text::Line> = self
            .categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let is_selected = self.selected_category == Some(i);
                let style = if is_selected {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(format!(" {} ", cat), style))
            })
            .collect();

        f.render_widget(
            ratatui::widgets::Paragraph::new(items).block(block),
            area,
        );
    }

    fn render_module_list(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Modules ")
            .borders(Borders::ALL);

        let items: Vec<Line> = self
            .modules
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let is_selected = self.selected_module == Some(i);
                let style = if is_selected {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                let rank_style = match m.rank.as_str() {
                    "excellent" => Style::default().add_modifier(Modifier::BOLD),
                    "manual" => Style::default().add_modifier(Modifier::DIM),
                    _ => Style::default(),
                };
                Line::from(vec![
                    Span::styled(format!(" {}", m.name), style),
                    Span::raw(" "),
                    Span::styled(format!("[{}]", m.rank), rank_style),
                ])
            })
            .collect();

        f.render_widget(
            ratatui::widgets::Paragraph::new(items).block(block),
            area,
        );
    }

    fn render_module_detail(&self, f: &mut Frame, area: Rect, module: &MsfModule) {
        let mut lines = vec![
            Line::from(Span::styled(
                format!(" Name:  {}", module.name),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(format!(" Rank:  {}", module.rank))),
            Line::from(Span::raw(format!(
                " Description:  {}",
                module.description
            ))),
        ];

        if let Some(date) = &module.disclosure {
            lines.push(Line::from(Span::raw(format!(" Disclosure:  {date}"))));
        }

        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(Span::styled(
            " [Enter] Back to list  [r] Run module  [i] View info  ",
            Style::default().add_modifier(Modifier::DIM),
        )));

        let block = Block::default()
            .title(" Module Details ")
            .borders(Borders::ALL);

        f.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
    }
}
