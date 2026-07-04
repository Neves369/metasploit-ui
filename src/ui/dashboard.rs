use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::msf::runner::check_msf_installed;

pub struct Dashboard {
    msf_installed: bool,
    msf_version: String,
    db_status: String,
    _system_info: Vec<(String, String)>,
}

impl Dashboard {
    pub fn new() -> Self {
        let (installed, version) = check_msf_installed();
        Self {
            msf_installed: installed,
            msf_version: version,
            db_status: "Not connected".to_string(),
            _system_info: Vec::new(),
        }
    }

    pub fn handle_key(&mut self, _key: crossterm::event::KeyEvent) -> bool {
        false
    }

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        _message: &str,
    ) -> String {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        self.render_welcome(f, chunks[0]);
        self.render_status(f, chunks[1]);
        self.render_quick_actions(f, chunks[2]);

        String::new()
    }

    fn render_welcome(&self, f: &mut Frame, area: Rect) {
        let lines = vec![
            Line::from(vec![
                Span::styled(
                    "  Metasploit TUI  ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::styled("  Status:  ", Style::default()),
                Span::styled(
                    if self.msf_installed {
                        "Installed"
                    } else {
                        "Not found"
                    },
                    Style::default().add_modifier(if self.msf_installed {
                        Modifier::BOLD
                    } else {
                        Modifier::DIM | Modifier::BOLD
                    }),
                ),
                Span::raw("  |  "),
                Span::styled("Version: ", Style::default()),
                Span::styled(&self.msf_version, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  |  "),
                Span::styled("DB: ", Style::default()),
                Span::styled(&self.db_status, Style::default().add_modifier(Modifier::BOLD)),
            ]),
        ];

        let block = Block::default()
            .title(" Dashboard ")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
        f.render_widget(paragraph, area);
    }

    fn render_status(&self, f: &mut Frame, area: Rect) {
        let module_counts = vec![
            ("exploit", "0"),
            ("auxiliary", "0"),
            ("payload", "0"),
            ("post", "0"),
            ("encoder", "0"),
            ("nop", "0"),
        ];

        let count_line: Vec<Span> = module_counts
            .iter()
            .flat_map(|(name, count)| {
                vec![
                    Span::styled(
                        format!(" {name}: "),
                        Style::default(),
                    ),
                    Span::styled(
                        *count,
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                ]
            })
            .collect();

        let lines = vec![
            Line::from(Span::styled(
                " Module Counts (run 'Refresh' to scan)  ",
                Style::default().add_modifier(Modifier::DIM),
            )),
            Line::from(count_line),
        ];

        let block = Block::default()
            .title(" Statistics ")
            .borders(Borders::ALL);

        f.render_widget(Paragraph::new(lines).block(block), area);
    }

    fn render_quick_actions(&self, f: &mut Frame, area: Rect) {
        let actions = vec![
            "[r]  Refresh module database     [u]  Update Metasploit",
            "[d]  Database: init/connect       [c]  Open Console",
            "[g]  Generate payload              [s]  Search modules",
        ];

        let lines: Vec<Line> = actions
            .iter()
            .map(|action| Line::from(Span::styled(*action, Style::default())))
            .collect();

        let block = Block::default()
            .title(" Quick Actions ")
            .borders(Borders::ALL);

        f.render_widget(Paragraph::new(lines).block(block), area);
    }
}
