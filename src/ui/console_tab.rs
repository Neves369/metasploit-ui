use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::msf::runner;

pub struct ConsoleTab {
    pub output: Vec<String>,
    pub input: String,
    pub cursor_pos: usize,
    pub history: Vec<String>,
    pub history_idx: Option<usize>,
    pub scroll_offset: usize,
    pub process_running: bool,
    pub feedback_message: String,
    pub command_running: bool,
    pub cmd_rx: Option<mpsc::Receiver<Result<String, String>>>,
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
            feedback_message: String::new(),
            command_running: false,
            cmd_rx: None,
        }
    }

    pub fn check_command(&mut self) {
        if !self.command_running {
            return;
        }
        if let Some(rx) = self.cmd_rx.as_ref() {
            match rx.try_recv() {
                Ok(result) => {
                    self.command_running = false;
                    self.cmd_rx = None;
                    match result {
                        Ok(out) => {
                            for line in out.lines() {
                                if !line.is_empty() {
                                    self.output.push(line.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            self.output.push(format!("[!] Error: {e}"));
                        }
                    }
                    self.output.push("---".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.command_running = false;
                    self.cmd_rx = None;
                    self.output.push("[!] Process disconnected".into());
                    self.output.push("---".into());
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.check_command();
        self.feedback_message.clear();

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
                            self.output.push("  exit/quit   - Close console".into());
                            self.output.push("  [Ctrl+D]    - Toggle msfconsole mode".into());
                            self.output.push("  [Ctrl+C]    - Interrupt".into());
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
                            self.output.push("---".into());
                        }
                        "exit" | "quit" => {}
                        _ => {
                            if self.process_running && !self.command_running {
                                let cmd = cmd.clone();
                                self.output
                                    .push(format!("[*] Running: msfconsole -x \"{cmd}\"..."));
                                self.command_running = true;
                                let (tx, rx) = mpsc::channel();
                                self.cmd_rx = Some(rx);
                                std::thread::spawn(move || {
                                    let result = runner::run_msf_command(&[&cmd, "exit"]);
                                    let _ = tx.send(result);
                                });
                                self.output.push("---".into());
                            } else if self.process_running && self.command_running {
                                self.output.push("[!] Command already running, please wait...".into());
                            } else {
                                self.output.push(format!("Unknown command: {cmd}"));
                                self.output
                                    .push("Type 'help' for available commands.".into());
                                self.output.push("---".into());
                            }
                        }
                    }
                }
                self.input.clear();
                self.cursor_pos = 0;
                self.scroll_offset = 0;
                true
            }
            KeyCode::Char(c) if key.modifiers == KeyModifiers::NONE => {
                self.input.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
                true
            }
            KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                if !self.process_running {
                    self.process_running = true;
                    self.output.push("[*] msfconsole started".into());
                } else {
                    self.process_running = false;
                    self.output.push("[*] msfconsole stopped".into());
                }
                self.output.push("---".into());
                true
            }
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                self.output.push("[!] Interrupt".into());
                self.output.push("---".into());
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
                    let idx = self
                        .history_idx
                        .map_or(self.history.len() - 1, |i| i.saturating_sub(1));
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
                self.scroll_offset = self.scroll_offset.saturating_add(10);
                true
            }
            KeyCode::PageDown => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
                true
            }
            _ => false,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) -> String {
        self.check_command();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);

        let mode_label = if self.process_running {
            if self.command_running {
                "[msfconsole][running...]"
            } else {
                "[msfconsole]"
            }
        } else {
            "[local]"
        };
        let output_block = Block::default()
            .title(format!(" Console {mode_label} "))
            .borders(Borders::ALL);

        let total = self.output.len();
        let visible_height = (chunks[0].height as usize).saturating_sub(2);
        let effective_offset = self.scroll_offset.min(total.saturating_sub(1));
        let start = if total > visible_height {
            total.saturating_sub(visible_height).saturating_sub(effective_offset)
        } else {
            0
        };
        let display_lines: Vec<Line> = self.output[start..]
            .iter()
            .map(|line| {
                if line.starts_with(">") {
                    Line::from(Span::styled(
                        line.as_str(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ))
                } else if line.starts_with("[!]") {
                    Line::from(Span::styled(
                        line.as_str(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ))
                } else if line.starts_with("[*]") {
                    Line::from(Span::styled(
                        line.as_str(),
                        Style::default().add_modifier(Modifier::DIM),
                    ))
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

        let input_paragraph =
            Paragraph::new(self.input.as_str()).block(input_block).style(Style::default());

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
