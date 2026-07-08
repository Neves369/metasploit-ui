use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::msf::msfconsole;

#[derive(Clone)]
pub struct SessionEntry {
    pub id: u32,
    pub session_type: String,
    pub target: String,
    pub status: String,
}

type SessionsResult = Result<Vec<SessionEntry>, String>;

enum PendingOp {
    None,
    Load(mpsc::Receiver<SessionsResult>),
    Kill(u32, usize, mpsc::Receiver<Result<String, String>>),
    Upgrade(u32, usize, mpsc::Receiver<Result<String, String>>),
}

pub struct Sessions {
    pub sessions: Vec<SessionEntry>,
    pub selected: Option<usize>,
    pub show_detail: bool,
    pub feedback_message: String,
    pub detail_info: Option<String>,
    pub loading: bool,
    pending: PendingOp,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            selected: None,
            show_detail: false,
            feedback_message: String::new(),
            detail_info: None,
            loading: false,
            pending: PendingOp::None,
        }
    }

    pub fn load_sessions(&mut self) {
        if self.loading {
            return;
        }
        self.loading = true;
        self.feedback_message = "Loading sessions...".to_string();
        let (tx, rx) = mpsc::channel();
        self.pending = PendingOp::Load(rx);
        std::thread::spawn(move || {
            let result = match msfconsole::list_sessions() {
                Ok(output) => {
                    let mut parsed = Vec::new();
                    for line in output.lines() {
                        let trimmed = line.trim();
                        if trimmed.is_empty()
                            || !trimmed.starts_with(|c: char| c.is_ascii_digit())
                        {
                            continue;
                        }
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(id) = parts[0].parse::<u32>() {
                                let target = parts.get(2).unwrap_or(&"").to_string();
                                parsed.push(SessionEntry {
                                    id,
                                    session_type: parts[1].to_string(),
                                    target,
                                    status: "unknown".to_string(),
                                });
                            }
                        }
                    }
                    Ok(parsed)
                }
                Err(e) => Err(e),
            };
            let _ = tx.send(result);
        });
    }

    pub fn check_pending(&mut self) {
        match std::mem::replace(&mut self.pending, PendingOp::None) {
            PendingOp::Load(rx) => {
                match rx.try_recv() {
                    Ok(result) => {
                        match result {
                            Ok(sessions) => {
                                if sessions.is_empty() {
                                    self.feedback_message = "No active sessions found".to_string();
                                } else {
                                    self.feedback_message.clear();
                                }
                                self.sessions = sessions;
                            }
                            Err(e) => {
                                self.feedback_message = format!("Failed to load sessions: {e}");
                            }
                        }
                        self.loading = false;
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        self.pending = PendingOp::Load(rx);
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.loading = false;
                        self.feedback_message = "Failed to load sessions: process disconnected".to_string();
                    }
                }
            }
            PendingOp::Kill(id, idx, rx) => {
                match rx.try_recv() {
                    Ok(result) => {
                        match result {
                            Ok(out) => {
                                self.feedback_message = format!("Killed session #{id}: {out}");
                                if idx < self.sessions.len() && self.sessions[idx].id == id {
                                    self.sessions.remove(idx);
                                }
                                self.selected = None;
                            }
                            Err(e) => {
                                self.feedback_message = format!("Failed to kill #{id}: {e}");
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        self.pending = PendingOp::Kill(id, idx, rx);
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.feedback_message = format!("Failed to kill #{id}: process disconnected");
                    }
                }
            }
            PendingOp::Upgrade(id, idx, rx) => {
                match rx.try_recv() {
                    Ok(result) => {
                        match result {
                            Ok(out) => {
                                self.feedback_message = format!("Upgraded session #{id}: {out}");
                            }
                            Err(e) => {
                                self.feedback_message = format!("Failed to upgrade #{id}: {e}");
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        self.pending = PendingOp::Upgrade(id, idx, rx);
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.feedback_message = format!("Failed to upgrade #{id}: process disconnected");
                    }
                }
            }
            PendingOp::None => {}
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.check_pending();
        self.feedback_message.clear();

        if self.show_detail {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.show_detail = false;
                    self.detail_info = None;
                    return true;
                }
                _ => return false,
            }
        }

        match key.code {
            KeyCode::Up => {
                let prev = self.selected.unwrap_or(0).saturating_sub(1);
                self.selected = if self.sessions.is_empty() {
                    None
                } else {
                    Some(prev)
                };
                true
            }
            KeyCode::Down => {
                if self.sessions.is_empty() {
                    return true;
                }
                let next = self
                    .selected
                    .map_or(0, |i| (i + 1).min(self.sessions.len() - 1));
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
                if !self.matches_pending(|p| matches!(p, PendingOp::Kill(..) | PendingOp::Upgrade(..)))
                    && !self.loading
                {
                    if let Some(idx) = self.selected {
                        if idx < self.sessions.len() {
                            let id = self.sessions[idx].id;
                            self.feedback_message = format!("Killing session #{id}...");
                            let (tx, rx) = mpsc::channel();
                            self.pending = PendingOp::Kill(id, idx, rx);
                            std::thread::spawn(move || {
                                let _ = tx.send(msfconsole::kill_session(id));
                            });
                        }
                    }
                }
                true
            }
            KeyCode::Char('i') => {
                if let Some(idx) = self.selected {
                    if idx < self.sessions.len() {
                        let id = self.sessions[idx].id;
                        self.feedback_message =
                            format!("Session #{id}: interactive mode not available in batch console");
                    }
                }
                true
            }
            KeyCode::Char('u') => {
                if !self.matches_pending(|p| matches!(p, PendingOp::Kill(..) | PendingOp::Upgrade(..)))
                    && !self.loading
                {
                    if let Some(idx) = self.selected {
                        if idx < self.sessions.len() {
                            let id = self.sessions[idx].id;
                            self.feedback_message = format!("Upgrading session #{id}...");
                            let (tx, rx) = mpsc::channel();
                            self.pending = PendingOp::Upgrade(id, idx, rx);
                            std::thread::spawn(move || {
                                let _ = tx.send(msfconsole::upgrade_session(id));
                            });
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn matches_pending(&self, f: fn(&PendingOp) -> bool) -> bool {
        f(&self.pending)
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        _message: &str,
    ) -> String {
        self.check_pending();

        if self.loading {
            let block = Block::default()
                .title(" Active Sessions ")
                .borders(Borders::ALL);
            let msg = Line::from(Span::styled(
                " Loading sessions... ",
                Style::default().add_modifier(Modifier::DIM),
            ));
            f.render_widget(Paragraph::new(vec![msg]).block(block), area);
            return "Loading sessions...".to_string();
        }

        if !self.feedback_message.is_empty() {
            return self.feedback_message.clone();
        }

        if self.show_detail {
            if let Some(idx) = self.selected {
                if idx < self.sessions.len() {
                    self.render_session_detail(f, area, &self.sessions[idx]);
                    return format!("Session #{}", self.sessions[idx].id);
                }
            }
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let block = Block::default()
            .title(" Active Sessions ")
            .borders(Borders::ALL);

        if self.sessions.is_empty() {
            let msg = Line::from(Span::styled(
                " No sessions. Switch to this tab to refresh.",
                Style::default().add_modifier(Modifier::DIM),
            ));
            f.render_widget(Paragraph::new(vec![msg]).block(block), chunks[0]);
        } else {
            let header = Line::from(vec![
                Span::styled(" ID  ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(" Type        ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    " Target          ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" Status  ", Style::default().add_modifier(Modifier::BOLD)),
            ]);

            let mut lines = vec![header];

            for (i, session) in self.sessions.iter().enumerate() {
                let is_sel = self.selected == Some(i);
                let style = if is_sel {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                let status_style = if session.status == "active" || session.status == "unknown" {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::DIM)
                };

                lines.push(Line::from(vec![
                    Span::styled(format!(" {:2}  ", session.id), style),
                    Span::styled(format!(" {:<11}", session.session_type), style),
                    Span::styled(format!(" {:<15}", session.target), style),
                    Span::styled(format!(" {:<7}", session.status), status_style),
                ]));
            }

            f.render_widget(Paragraph::new(lines).block(block), chunks[0]);
        }

        let hint_block = Block::default().borders(Borders::ALL);

        let hint = Line::from(Span::styled(
            " [↑/↓] Navigate  [Enter] Details  [k] Kill  [u] Upgrade  ",
            Style::default().add_modifier(Modifier::DIM),
        ));

        f.render_widget(Paragraph::new(hint).block(hint_block), chunks[1]);

        if let Some(idx) = self.selected {
            if idx < self.sessions.len() {
                return format!(
                    "Session #{} - {}",
                    self.sessions[idx].id, self.sessions[idx].target
                );
            }
        }
        String::new()
    }

    fn render_session_detail(
        &self,
        f: &mut Frame,
        area: Rect,
        session: &SessionEntry,
    ) {
        let lines = vec![
            Line::from(Span::styled(
                format!(" Session #{} ", session.id),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(format!(" Type:   {}", session.session_type))),
            Line::from(Span::raw(format!(" Target: {}", session.target))),
            Line::from(Span::raw(format!(" Status: {}", session.status))),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                " [i] Interact  [u] Upgrade to meterpreter  [k] Kill  [Enter] Back  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
        ];

        let block = Block::default()
            .title(" Session Details ")
            .borders(Borders::ALL);

        f.render_widget(
            Paragraph::new(lines).block(block).wrap(Wrap { trim: false }),
            area,
        );
    }
}
