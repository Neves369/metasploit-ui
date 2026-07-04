use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

#[derive(Clone)]
pub struct SessionEntry {
    pub id: u32,
    pub session_type: String,
    pub target: String,
    pub port: u16,
    pub status: String,
    pub uptime: String,
}

pub struct Sessions {
    pub sessions: Vec<SessionEntry>,
    pub selected: Option<usize>,
    pub show_detail: bool,
}

impl Sessions {
    pub fn new() -> Self {
        let sessions = vec![
            SessionEntry {
                id: 1,
                session_type: "meterpreter".into(),
                target: "192.168.1.105".into(),
                port: 4444,
                status: "active".into(),
                uptime: "00:15:23".into(),
            },
            SessionEntry {
                id: 2,
                session_type: "shell".into(),
                target: "10.0.0.15".into(),
                port: 8080,
                status: "active".into(),
                uptime: "00:02:01".into(),
            },
            SessionEntry {
                id: 3,
                session_type: "meterpreter".into(),
                target: "192.168.1.200".into(),
                port: 443,
                status: "dead".into(),
                uptime: "00:00:00".into(),
            },
        ];

        Self {
            sessions,
            selected: None,
            show_detail: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.show_detail {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.show_detail = false;
                    return true;
                }
                _ => return false,
            }
        }

        match key.code {
            KeyCode::Up => {
                let prev = self.selected.unwrap_or(0).saturating_sub(1);
                self.selected = if self.sessions.is_empty() { None } else { Some(prev) };
                true
            }
            KeyCode::Down => {
                if self.sessions.is_empty() {
                    return true;
                }
                let next = self.selected.map_or(0, |i| (i + 1).min(self.sessions.len() - 1));
                self.selected = Some(next);
                true
            }
            KeyCode::Enter => {
                if self.selected.is_some() {
                    self.show_detail = true;
                }
                true
            }
            KeyCode::Char('k') => {
                if let Some(idx) = self.selected {
                    if idx < self.sessions.len() {
                        self.sessions[idx].status = "dead".into();
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
        if self.show_detail && self.selected.is_some() {
            let idx = self.selected.unwrap();
            if idx < self.sessions.len() {
                self.render_session_detail(f, area, &self.sessions[idx]);
                return format!("Session #{} details", self.sessions[idx].id);
            }
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let block = Block::default()
            .title(" Active Sessions ")
            .borders(Borders::ALL);

        let header = Line::from(vec![
            Span::styled(" ID  ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Type        ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Target          ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Port  ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Status  ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(" Uptime   ", Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let mut lines = vec![header];

        for (i, session) in self.sessions.iter().enumerate() {
            let is_sel = self.selected == Some(i);
            let style = if is_sel {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            let status_style = if session.status == "active" {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default().add_modifier(Modifier::DIM)
            };

            lines.push(Line::from(vec![
                Span::styled(format!(" {:2}  ", session.id), style),
                Span::styled(format!(" {:<11}", session.session_type), style),
                Span::styled(format!(" {:<15}", session.target), style),
                Span::styled(format!(" {:5}  ", session.port), style),
                Span::styled(format!(" {:<7}", session.status), status_style),
                Span::styled(format!(" {}", session.uptime), style),
            ]));
        }

        f.render_widget(Paragraph::new(lines).block(block), chunks[0]);

        let hint_block = Block::default()
            .borders(Borders::ALL);

        let hint = Line::from(Span::styled(
            " [↑/↓] Navigate  [Enter] Details  [k] Kill session  ",
            Style::default().add_modifier(Modifier::DIM),
        ));

        f.render_widget(Paragraph::new(hint).block(hint_block), chunks[1]);

        if let Some(idx) = self.selected {
            if idx < self.sessions.len() {
                return format!("Session #{} - {}", self.sessions[idx].id, self.sessions[idx].target);
            }
        }
        String::new()
    }

    fn render_session_detail(&self, f: &mut Frame, area: Rect, session: &SessionEntry) {
        let lines = vec![
            Line::from(Span::styled(
                format!(" Session #{} ", session.id),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(format!(" Type:   {}", session.session_type))),
            Line::from(Span::raw(format!(" Target: {}", session.target))),
            Line::from(Span::raw(format!(" Port:   {}", session.port))),
            Line::from(Span::raw(format!(" Status: {}", session.status))),
            Line::from(Span::raw(format!(" Uptime: {}", session.uptime))),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                " [i] Interact  [u] Upgrade to meterpreter  [k] Kill  [Enter] Back  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
        ];

        let block = Block::default()
            .title(" Session Details ")
            .borders(Borders::ALL);

        f.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
    }
}
