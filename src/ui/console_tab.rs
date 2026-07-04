use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub struct ConsoleTab {
    pub output: Vec<String>,
    pub input: String,
    pub cursor_pos: usize,
    pub history: Vec<String>,
    pub history_idx: Option<usize>,
    pub scroll_offset: usize,
    pub process_running: bool,
}

impl ConsoleTab {
    pub fn new() -> Self {
        let output = vec![
            "Metasploit TUI Console v0.1".into(),
            "Type commands below. Press Ctrl+D to start msfconsole.".into(),
            "---".into(),
        ];

        Self {
            output,
            input: String::new(),
            cursor_pos: 0,
            history: Vec::new(),
            history_idx: None,
            scroll_offset: 0,
            process_running: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => {
                let cmd = self.input.trim().to_string();
                if !cmd.is_empty() {
                    self.output.push(format!("> {cmd}"));
                    self.history.push(cmd.clone());
                    self.history_idx = None;

                    match cmd.as_str() {
                        "clear" | "cls" => {
                            self.output.clear();
                            self.output.push("Metasploit TUI Console v0.1".into());
                            self.output.push("---".into());
                        }
                        "help" => {
                            self.output.push("Available commands:".into());
                            self.output.push("  help        - Show this help".into());
                            self.output.push("  clear/cls   - Clear console".into());
                            self.output.push("  msfconsole  - Start Metasploit console".into());
                            self.output.push("  version     - Show version".into());
                            self.output.push("  exit/quit   - Close console (use Ctrl+D to send to msf)".into());
                            self.output.push("---".into());
                        }
                        "version" => {
                            self.output.push("metasploit-tui v0.1".into());
                            self.output.push("---".into());
                        }
                        "msfconsole" => {
                            self.process_running = true;
                            self.output.push("[*] Starting msfconsole...".into());
                            self.output.push("[*] Use Ctrl+D to send commands, Ctrl+C to interrupt".into());
                            self.output.push("[!] msfconsole integration not yet connected".into());
                            self.output.push("---".into());
                        }
                        "exit" | "quit" => {}
                        _ => {
                            if self.process_running {
                                self.output.push(format!("[msf] {cmd}"));
                            } else {
                                self.output.push(format!("Unknown command: {cmd}"));
                                self.output.push("Type 'help' for available commands.".into());
                            }
                            self.output.push("---".into());
                        }
                    }
                }
                self.input.clear();
                self.cursor_pos = 0;
                let total = self.output.len();
                if total > 0 {
                    self.scroll_offset = total.saturating_sub(20);
                }
                true
            }
            KeyCode::Char(c) if key.modifiers == KeyModifiers::NONE => {
                self.input.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
                true
            }
            KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                self.process_running = !self.process_running;
                if self.process_running {
                    self.output.push("[*] msfconsole started".into());
                } else {
                    self.output.push("[*] msfconsole stopped".into());
                }
                self.output.push("---".into());
                true
            }
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                if self.process_running {
                    self.output.push("[!] Interrupt sent to msfconsole".into());
                    self.output.push("---".into());
                }
                true
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.input.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
                true
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.input.len() {
                    self.input.remove(self.cursor_pos);
                }
                true
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
                true
            }
            KeyCode::Right => {
                if self.cursor_pos < self.input.len() {
                    self.cursor_pos += 1;
                }
                true
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
                true
            }
            KeyCode::End => {
                self.cursor_pos = self.input.len();
                true
            }
            KeyCode::Up => {
                if !self.history.is_empty() {
                    let idx = self.history_idx.map_or(self.history.len() - 1, |i| i.saturating_sub(1));
                    self.history_idx = Some(idx);
                    self.input = self.history[idx].clone();
                    self.cursor_pos = self.input.len();
                }
                true
            }
            KeyCode::Down => {
                if let Some(idx) = self.history_idx {
                    let new_idx = idx + 1;
                    if new_idx < self.history.len() {
                        self.history_idx = Some(new_idx);
                        self.input = self.history[new_idx].clone();
                    } else {
                        self.history_idx = None;
                        self.input.clear();
                    }
                    self.cursor_pos = self.input.len();
                }
                true
            }
            KeyCode::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
                true
            }
            KeyCode::PageDown => {
                self.scroll_offset += 10;
                true
            }
            _ => false,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) -> String {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);

        let output_block = Block::default()
            .title(format!(
                " Console {} ",
                if self.process_running { "[msfconsole]" } else { "[local]" }
            ))
            .borders(Borders::ALL);

        let total = self.output.len();
        let visible_height = (chunks[0].height as usize).saturating_sub(2);
        let start = if total > visible_height {
            total.saturating_sub(visible_height)
        } else {
            0
        };
        let display_lines: Vec<Line> = self.output[start..]
            .iter()
            .map(|line| {
                if line.starts_with(">") {
                    Line::from(Span::styled(line.as_str(), Style::default().add_modifier(Modifier::BOLD)))
                } else if line.starts_with("[!]") {
                    Line::from(Span::styled(line.as_str(), Style::default().add_modifier(Modifier::BOLD)))
                } else if line.starts_with("[*]") {
                    Line::from(Span::styled(line.as_str(), Style::default().add_modifier(Modifier::DIM)))
                } else {
                    Line::from(Span::raw(line.as_str()))
                }
            })
            .collect();

        let output_paragraph = Paragraph::new(display_lines)
            .block(output_block)
            .wrap(Wrap { trim: false });

        f.render_widget(output_paragraph, chunks[0]);

        let input_block = Block::default()
            .title(" Input ")
            .borders(Borders::ALL);

        let input_paragraph = Paragraph::new(self.input.as_str())
            .block(input_block)
            .style(Style::default());

        f.render_widget(input_paragraph, chunks[1]);

        let x = chunks[1].x + 1 + self.cursor_pos as u16;
        let y = chunks[1].y + 1;
        if x < chunks[1].right() && y < chunks[1].bottom() {
            f.set_cursor_position((x, y));
        }

        if self.process_running {
            "Console: msfconsole active".to_string()
        } else {
            String::new()
        }
    }
}
