use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::msf::runner::{self, HealthCheckResult};

const MSF_BANNER: &[&str] = &[
    "  ███╗   ███╗ ███████╗ ███████╗",
    "  ████╗ ████║ ██╔════╝ ██╔════╝",
    "  ██╔████╔██║ ███████╗ ███████╗",
    "  ██║╚██╔╝██║ ╚════██║ ╚════██║",
    "  ██║ ╚═╝ ██║ ███████║ ███████║",
    "  ╚═╝     ╚═╝ ╚══════╝ ╚══════╝",
];

pub struct Dashboard {
    pub health: Option<HealthCheckResult>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            health: Some(runner::quick_check()),
        }
    }

    pub fn run_health_check(&mut self) {
        self.health = Some(runner::run_health_check());
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        match key.code {
            crossterm::event::KeyCode::Char('h') => {
                self.run_health_check();
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
        let has_health = self.health.is_some();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Length(if has_health { 7 } else { 2 }),
                Constraint::Length(if has_health { 4 } else { 0 }),
                Constraint::Min(0),
            ])
            .split(area);

        self.render_banner(f, chunks[0]);
        self.render_health(f, chunks[1]);
        if has_health {
            self.render_module_counts(f, chunks[2]);
        }
        self.render_quick_actions(f, chunks[3], has_health);

        String::new()
    }

    fn render_banner(&self, f: &mut Frame, area: Rect) {
        let lines: Vec<Line> = MSF_BANNER
            .iter()
            .map(|line| {
                Line::from(Span::styled(
                    *line,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default()),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, area);
    }

    fn render_health(&self, f: &mut Frame, area: Rect) {
        let lines = match &self.health {
            None => vec![
                Line::from(Span::raw("")),
                Line::from(Span::styled(
                    "  Press [h] to run system health check  ",
                    Style::default().add_modifier(Modifier::DIM),
                )),
            ],
            Some(h) => {
                let mut items = Vec::new();
                items.push(self.status_line("msfconsole", &h.msf_version));
                items.push(self.status_line("msfvenom  ", &h.msfvenom_version));
                items.push(self.status_line("ruby      ", &h.ruby_version));
                items.push(self.status_line("database  ", &h.db_status));
                items
            }
        };

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" System Health ")
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, area);
    }

    fn status_line<'a>(&self, label: &'a str, result: &'a (bool, String)) -> Line<'a> {
        let (ok, detail) = result;
        let icon = if *ok { " ✔ " } else { " ✘ " };
        Line::from(vec![
            Span::raw("  "),
            Span::styled(icon, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(label, Style::default().add_modifier(Modifier::DIM)),
            Span::raw("  "),
            Span::styled(detail.as_str(), Style::default().add_modifier(Modifier::BOLD)),
        ])
    }

    fn render_module_counts(&self, f: &mut Frame, area: Rect) {
        let pad = |s: &str, w: usize| -> String {
            if s.len() >= w {
                s.to_string()
            } else {
                format!("{}{}", s, " ".repeat(w - s.len()))
            }
        };

        let counts = match &self.health {
            Some(h) => &h.module_counts,
            None => return,
        };

        let to_line = |items: &[(String, usize)]| -> Line {
            let mut spans = Vec::new();
            for (cat, count) in items {
                spans.push(Span::styled(
                    format!(" {}: ", pad(cat, 10)),
                    Style::default().add_modifier(Modifier::DIM),
                ));
                spans.push(Span::styled(
                    format!("{:>4}", count),
                    Style::default().add_modifier(Modifier::BOLD),
                ));
                spans.push(Span::raw("  "));
            }
            Line::from(spans)
        };

        let (first, second) = counts.split_at(4);
        let lines = vec![to_line(first), to_line(second)];

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Module Counts ")
                    .borders(Borders::ALL),
            );
        f.render_widget(paragraph, area);
    }

    fn render_quick_actions(&self, f: &mut Frame, area: Rect, has_health: bool) {
        let mut actions = vec![
            ("[h]", "Health check"),
            ("[r]", "Refresh modules"),
            ("[u]", "Update Metasploit"),
            ("[d]", "Database: init/connect"),
            ("[g]", "Generate payload"),
            ("[s]", "Search modules"),
        ];

        if !has_health {
            actions.insert(0, ("[h]", "Run health check"));
        }

        let lines: Vec<Line> = actions
            .chunks(3)
            .map(|chunk| {
                let spans: Vec<Span> = chunk
                    .iter()
                    .flat_map(|(key, desc)| {
                        vec![
                            Span::styled(
                                format!(" {key} "),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(*desc, Style::default().add_modifier(Modifier::DIM)),
                            Span::raw("   "),
                        ]
                    })
                    .collect();
                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Quick Actions ")
                    .borders(Borders::ALL),
            );
        f.render_widget(paragraph, area);
    }
}
