use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::components::input::InputField;
use crate::msf::msfvenom;

pub struct PayloadGen {
    pub fields: Vec<InputField>,
    pub focus: usize,
    pub output: Vec<String>,
    pub show_preview: bool,
    pub generating: bool,
    pub payload_rx: Option<mpsc::Receiver<Result<String, String>>>,
}

impl PayloadGen {
    pub fn new() -> Self {
        let fields = vec![
            InputField::new("Payload", "linux/x64/meterpreter/reverse_tcp"),
            InputField::new("LHOST", ""),
            InputField::new("LPORT", "4444"),
            InputField::new("Format", "elf"),
            InputField::new("Encoder", ""),
            InputField::new("Iterations", "1"),
            InputField::new("Platform", "linux"),
            InputField::new("Arch", "x64"),
            InputField::new("Output", "./payload.elf"),
            InputField::new("Extra options", ""),
        ];

        Self {
            fields,
            focus: 0,
            output: Vec::new(),
            show_preview: false,
            generating: false,
            payload_rx: None,
        }
    }

    fn build_options(&self) -> msfvenom::PayloadOptions {
        let mut opts = msfvenom::PayloadOptions::default();
        for field in &self.fields {
            if field.value.is_empty() {
                continue;
            }
            match field.label.as_str() {
                "Payload" => opts.payload = field.value.clone(),
                "LHOST" => opts.lhost = Some(field.value.clone()),
                "LPORT" => opts.lport = Some(field.value.clone()),
                "Format" => opts.format = Some(field.value.clone()),
                "Encoder" => opts.encoder = Some(field.value.clone()),
                "Iterations" => opts.iterations = field.value.parse().ok(),
                "Platform" => opts.platform = Some(field.value.clone()),
                "Arch" => opts.arch = Some(field.value.clone()),
                "Output" => opts.output = Some(field.value.clone()),
                "Extra options" => opts.extra = field.value.split_whitespace().map(String::from).collect(),
                _ => {}
            }
        }
        opts
    }

    fn generate_preview(&self) -> String {
        let mut cmd = String::from("msfvenom");
        for field in &self.fields {
            if field.value.is_empty() {
                continue;
            }
            match field.label.as_str() {
                "Payload" => cmd.push_str(&format!(" -p {}", field.value)),
                "LHOST" => cmd.push_str(&format!(" LHOST={}", field.value)),
                "LPORT" => cmd.push_str(&format!(" LPORT={}", field.value)),
                "Format" => cmd.push_str(&format!(" -f {}", field.value)),
                "Encoder" => cmd.push_str(&format!(" -e {}", field.value)),
                "Iterations" => cmd.push_str(&format!(" -i {}", field.value)),
                "Platform" => cmd.push_str(&format!(" --platform {}", field.value)),
                "Arch" => cmd.push_str(&format!(" -a {}", field.value)),
                "Output" => cmd.push_str(&format!(" -o {}", field.value)),
                "Extra options" => cmd.push_str(&format!(" {}", field.value)),
                _ => {}
            }
        }
        cmd
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.check_generation();
        let current_focus = self.focus;
        let field_count = self.fields.len();

        if self.fields[current_focus].focused {
            match key.code {
                KeyCode::Esc => {
                    self.fields[current_focus].focused = false;
                    return true;
                }
                KeyCode::Char(c) => {
                    self.fields[current_focus].insert_char(c);
                    self.show_preview = false;
                    return true;
                }
                KeyCode::Backspace => {
                    self.fields[current_focus].delete_char();
                    self.show_preview = false;
                    return true;
                }
                KeyCode::Delete => {
                    self.fields[current_focus].delete_forward();
                    self.show_preview = false;
                    return true;
                }
                KeyCode::Left => {
                    self.fields[current_focus].move_left();
                    return true;
                }
                KeyCode::Right => {
                    self.fields[current_focus].move_right();
                    return true;
                }
                KeyCode::Home => {
                    self.fields[current_focus].move_home();
                    return true;
                }
                KeyCode::End => {
                    self.fields[current_focus].move_end();
                    return true;
                }
                KeyCode::Tab | KeyCode::Down | KeyCode::Enter => {
                    self.fields[current_focus].focused = false;
                    let next = (current_focus + 1).min(field_count - 1);
                    self.fields[next].focused = true;
                    self.focus = next;
                    return true;
                }
                KeyCode::BackTab | KeyCode::Up => {
                    self.fields[current_focus].focused = false;
                    let prev = current_focus.saturating_sub(1);
                    self.fields[prev].focused = true;
                    self.focus = prev;
                    return true;
                }
                _ => return false,
            }
        }

        match key.code {
            KeyCode::Char('/') => {
                self.fields[self.focus].focused = true;
                true
            }
            KeyCode::Tab | KeyCode::Down => {
                let next = (self.focus + 1) % field_count;
                self.fields[next].focused = true;
                self.focus = next;
                true
            }
            KeyCode::BackTab | KeyCode::Up => {
                let prev = if self.focus == 0 {
                    field_count - 1
                } else {
                    self.focus - 1
                };
                self.fields[prev].focused = true;
                self.focus = prev;
                true
            }
            KeyCode::Enter => {
                if !self.fields.iter().any(|f| f.focused) {
                    self.show_preview = !self.show_preview;
                    if self.show_preview {
                        let preview = self.generate_preview();
                        self.output.push(format!("$ {}", preview));
                    }
                }
                true
            }
            KeyCode::Char('c') => {
                self.output.clear();
                true
            }
            KeyCode::Char('g') => {
                if !self.generating {
                    let opts = self.build_options();
                    let preview = self.generate_preview();
                    self.output.push(format!("$ {preview}"));
                    self.generating = true;
                    let (tx, rx) = mpsc::channel();
                    self.payload_rx = Some(rx);
                    std::thread::spawn(move || {
                        let result = msfvenom::generate_payload(&opts);
                        let _ = tx.send(result);
                    });
                }
                true
            }
            KeyCode::Esc => {
                true
            }
            _ => false,
        }
    }

    pub fn check_generation(&mut self) {
        if !self.generating {
            return;
        }
        if let Some(rx) = self.payload_rx.as_ref() {
            match rx.try_recv() {
                Ok(result) => {
                    self.generating = false;
                    self.payload_rx = None;
                    match result {
                        Ok(output) => {
                            for line in output.lines() {
                                self.output.push(line.to_string());
                            }
                            self.output.push("---".to_string());
                        }
                        Err(e) => {
                            self.output.push(format!("Error: {e}"));
                            self.output.push("---".to_string());
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.generating = false;
                    self.payload_rx = None;
                    self.output.push("Error: process disconnected".to_string());
                    self.output.push("---".to_string());
                }
            }
        }
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        _message: &str,
    ) -> String {
        self.check_generation();
        let has_output = !self.output.is_empty();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(if self.show_preview { 6 } else { 3 }),
                if has_output {
                    Constraint::Length(
                        (self.output.len() as u16).min(10).saturating_add(2),
                    )
                } else {
                    Constraint::Length(0)
                },
            ])
            .split(area);

        let form_area = chunks[0];

        let field_count = self.fields.len();
        let rows = (field_count + 1) / 2;
        let constraints: Vec<Constraint> = std::iter::repeat(Constraint::Length(3))
            .take(rows)
            .collect();

        let form_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(form_area);

        for (row, chunk) in form_chunks.iter().enumerate() {
            let left = row * 2;
            let right = left + 1;

            let row_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(*chunk);

            if left < field_count {
                self.fields[left].render(f, row_chunks[0]);
            }
            if right < field_count {
                self.fields[right].render(f, row_chunks[1]);
            }
        }

        let preview_block = Block::default()
            .title(" Preview ")
            .borders(Borders::ALL);

        if self.generating {
            let lines = vec![
                Line::from(Span::styled(
                    " Generating payload... ",
                    Style::default().add_modifier(Modifier::DIM),
                )),
            ];
            f.render_widget(
                Paragraph::new(lines).block(preview_block).wrap(Wrap { trim: false }),
                chunks[1],
            );
        } else if self.show_preview {
            let preview_cmd = self.generate_preview();
            let lines = vec![
                Line::from(Span::styled(
                    &preview_cmd,
                    Style::default().add_modifier(Modifier::BOLD),
                )),
            ];
            f.render_widget(
                Paragraph::new(lines).block(preview_block).wrap(Wrap { trim: false }),
                chunks[1],
            );
        } else {
            let lines = vec![
                Line::from(Span::styled(
                    " [Enter] Preview  [Tab] Next  [/] Edit  [g] Generate  [c] Clear  ",
                    Style::default().add_modifier(Modifier::DIM),
                )),
            ];
            f.render_widget(Paragraph::new(lines).block(preview_block), chunks[1]);
        }

        if has_output && chunks.len() > 2 {
            let output_block = Block::default()
                .title(" Output ")
                .borders(Borders::ALL);

            let output_lines: Vec<Line> = self
                .output
                .iter()
                .rev()
                .take(8)
                .rev()
                .map(|line| Line::from(Span::raw(line.as_str())))
                .collect();

            f.render_widget(
                Paragraph::new(output_lines)
                    .block(output_block)
                    .wrap(Wrap { trim: false }),
                chunks[2],
            );
        }

        String::new()
    }
}
