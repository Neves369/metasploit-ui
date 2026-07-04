use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::Terminal;

use crate::components::status_bar::StatusBar;
use crate::components::tab_bar::TabBar;
use crate::ui::console_tab::ConsoleTab;
use crate::ui::dashboard::Dashboard;
use crate::ui::explorer::Explorer;
use crate::ui::payload_gen::PayloadGen;
use crate::ui::resources::Resources;
use crate::ui::sessions::Sessions;

pub struct App {
    pub current_tab: usize,
    pub should_quit: bool,
    pub status_message: String,
    pub show_help: bool,

    pub tab_bar: TabBar,
    pub status_bar: StatusBar,

    pub dashboard: Dashboard,
    pub explorer: Explorer,
    pub payload_gen: PayloadGen,
    pub sessions: Sessions,
    pub console_tab: ConsoleTab,
    pub resources: Resources,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: 0,
            should_quit: false,
            status_message: String::new(),
            show_help: false,

            tab_bar: TabBar::new(),
            status_bar: StatusBar::new(),

            dashboard: Dashboard::new(),
            explorer: Explorer::new(),
            payload_gen: PayloadGen::new(),
            sessions: Sessions::new(),
            console_tab: ConsoleTab::new(),
            resources: Resources::new(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if self.should_quit {
                break;
            }

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => self.handle_key(key),
                    Event::Resize(..) => {}
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.show_help {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                self.show_help = false;
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') if key.modifiers == KeyModifiers::NONE => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Tab => {
                self.current_tab = (self.current_tab + 1) % 6;
                self.status_message.clear();
            }
            KeyCode::BackTab => {
                self.current_tab = if self.current_tab == 0 {
                    5
                } else {
                    self.current_tab - 1
                };
                self.status_message.clear();
            }
            KeyCode::Char(c) if c.is_ascii_digit() && c > '0' && c <= '6' => {
                self.current_tab = (c as u8 - b'1') as usize;
                self.status_message.clear();
            }
            _ => {
                let consumed = match self.current_tab {
                    0 => self.dashboard.handle_key(key),
                    1 => self.explorer.handle_key(key),
                    2 => self.payload_gen.handle_key(key),
                    3 => self.sessions.handle_key(key),
                    4 => self.console_tab.handle_key(key),
                    5 => self.resources.handle_key(key),
                    _ => false,
                };
                if !consumed {
                    self.status_message = format!("Unbound key: {:?}", key.code);
                } else {
                    self.status_message.clear();
                }
            }
        }
    }

    fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        self.tab_bar.render(f, chunks[0], self.current_tab);

        let content = match self.current_tab {
            0 => self.dashboard.render(f, chunks[1], &self.status_message),
            1 => self.explorer.render(f, chunks[1], &self.status_message),
            2 => self.payload_gen.render(f, chunks[1], &self.status_message),
            3 => self.sessions.render(f, chunks[1], &self.status_message),
            4 => self.console_tab.render(f, chunks[1]),
            5 => self.resources.render(f, chunks[1], &self.status_message),
            _ => String::new(),
        };

        self.status_bar.render(f, chunks[2], self.current_tab, &content);

        if self.show_help {
            self.render_help(f);
        }
    }

    fn render_help(&self, f: &mut Frame) {
        let area = f.area();
        let help_rect = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        let help_items = vec![
            ("[1-6]", "Switch tab"),
            ("[Tab]", "Next tab"),
            ("[Shift+Tab]", "Previous tab"),
            ("[q/Ctrl+c]", "Quit"),
            ("[?]", "Toggle help"),
            ("[Esc]", "Close help"),
        ];

        let help_text: Vec<ratatui::text::Line> = help_items
            .iter()
            .map(|(key, desc)| {
                ratatui::text::Line::from(vec![
                    ratatui::text::Span::styled(
                        *key,
                        ratatui::style::Style::default()
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    ),
                    ratatui::text::Span::raw(format!("  {desc}")),
                ])
            })
            .collect();

        let block = ratatui::widgets::Block::default()
            .title(" Help ")
            .borders(ratatui::widgets::Borders::ALL)
            .style(ratatui::style::Style::default());

        let paragraph = ratatui::widgets::Paragraph::new(help_text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(ratatui::widgets::Clear, help_rect);
        f.render_widget(paragraph, help_rect);
    }
}
