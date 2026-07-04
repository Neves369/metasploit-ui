use std::collections::{HashMap, HashSet};
use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::components::input::InputField;
use crate::msf::msfconsole;
use crate::msf::parser;
use crate::msf::runner;

type ModuleLoadResult = (String, Result<Vec<parser::ModuleInfo>, String>);

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
    pub module_cache: HashMap<String, Vec<MsfModule>>,
    pub displayed_modules: Vec<MsfModule>,
    pub selected_module: Option<usize>,
    pub scroll_offset: usize,
    pub visible_height: usize,
    pub search_input: InputField,
    pub show_detail: bool,
    pub feedback_message: String,
    pub full_info_content: Option<String>,
    pub loading_categories: HashSet<String>,
    pub loading_failed: Vec<String>,
    pub rx: Option<mpsc::Receiver<ModuleLoadResult>>,
    pub all_loaded: bool,
    pub run_running: bool,
    pub run_rx: Option<mpsc::Receiver<Result<String, String>>>,
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

        let (tx, rx) = mpsc::channel();
        let loading_categories: HashSet<String> = categories.iter().skip(1).cloned().collect();

        for cat in categories.iter().skip(1) {
            let tx = tx.clone();
            let cat = cat.clone();
            std::thread::spawn(move || {
                let result = msfconsole::show_modules(&cat);
                let _ = tx.send((cat, result));
            });
        }

        Self {
            module_cache: HashMap::new(),
            displayed_modules: Vec::new(),
            selected_category: Some(0),
            selected_module: None,
            scroll_offset: 0,
            visible_height: 20,
            search_input: InputField::new("Search", ""),
            show_detail: false,
            feedback_message: String::new(),
            full_info_content: None,
            categories,
            loading_categories,
            loading_failed: Vec::new(),
            rx: Some(rx),
            all_loaded: false,
            run_running: false,
            run_rx: None,
        }
    }

    pub fn check_loading(&mut self) {
        if self.all_loaded {
            return;
        }

        let rx = match self.rx.as_ref() {
            Some(rx) => rx,
            None => return,
        };

        loop {
            match rx.try_recv() {
                Ok((category, result)) => {
                    self.loading_categories.remove(&category);
                    match result {
                        Ok(modules) => {
                            let msf_modules: Vec<MsfModule> = modules
                                .into_iter()
                                .map(|m| MsfModule {
                                    name: m.name,
                                    rank: m.rank,
                                    description: m.description,
                                    disclosure: None,
                                })
                                .collect();
                            let count = msf_modules.len();
                            self.module_cache.insert(category.clone(), msf_modules);
                            self.feedback_message =
                                format!("Loaded {category} ({count} modules)");
                        }
                        Err(e) => {
                            self.loading_failed
                                .push(format!("{category}: {e}"));
                            self.feedback_message =
                                format!("Failed to load {category}: {e}");
                        }
                    }

                    if let Some(idx) = self.selected_category {
                        if idx < self.categories.len() && self.categories[idx] == category {
                            if let Some(cached) = self.module_cache.get(&category) {
                                self.displayed_modules = cached.clone();
                                self.selected_module = None;
                                self.scroll_offset = 0;
                            }
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    for cat in self.loading_categories.drain() {
                        self.loading_failed
                            .push(format!("{cat}: loading failed unexpectedly"));
                        self.feedback_message =
                            format!("Failed to load {cat}");
                    }
                    break;
                }
            }
        }

        if self.loading_categories.is_empty() && !self.all_loaded {
            self.all_loaded = true;

            let mut all = Vec::new();
            for mods in self.module_cache.values() {
                all.extend(mods.iter().cloned());
            }
            let total = all.len();
            self.module_cache.insert("All".to_string(), all);

            let failed_count = self.loading_failed.len();
            if failed_count > 0 {
                self.feedback_message = format!(
                    "Loaded {total} modules ({failed_count} categories failed)"
                );
            } else {
                self.feedback_message = format!("Loaded {total} modules");
            }

            if let Some(idx) = self.selected_category {
                if idx < self.categories.len() && self.categories[idx] == "All" {
                    if let Some(cached) = self.module_cache.get("All") {
                        self.displayed_modules = cached.clone();
                        self.selected_module = None;
                        self.scroll_offset = 0;
                    }
                }
            }
        }
    }

    pub fn load_modules(&mut self, category: &str) {
        self.scroll_offset = 0;
        self.selected_module = None;
        self.show_detail = false;
        self.full_info_content = None;

        if let Some(cached) = self.module_cache.get(category) {
            self.displayed_modules = cached.clone();
            return;
        }

        self.displayed_modules.clear();
    }

    fn apply_search_filter(&mut self) {
        let query = self.search_input.value.to_lowercase();
        if query.is_empty() {
            if let Some(idx) = self.selected_category {
                if idx < self.categories.len() {
                    let cat = self.categories[idx].clone();
                    self.load_modules(&cat);
                }
            }
            return;
        }

        let Some(idx) = self.selected_category else { return };
        if idx >= self.categories.len() {
            return;
        }

        let cat = &self.categories[idx];
        let source = match self.module_cache.get(cat.as_str()) {
            Some(v) => v,
            None => return,
        };

        self.displayed_modules = source
            .iter()
            .filter(|m| m.name.to_lowercase().contains(&query))
            .cloned()
            .collect();
        self.selected_module = None;
        self.scroll_offset = 0;
    }

    fn check_run(&mut self) {
        if !self.run_running {
            return;
        }
        if let Some(rx) = self.run_rx.as_ref() {
            match rx.try_recv() {
                Ok(result) => {
                    self.run_running = false;
                    self.run_rx = None;
                    match result {
                        Ok(output) => {
                            let trimmed: Vec<&str> = output
                                .lines()
                                .filter(|l| {
                                    !l.contains("msf6")
                                        && !l.is_empty()
                                        && !l.contains("[-]")
                                })
                                .collect();
                            let text = if trimmed.len() > 5 {
                                format!("{} (...)", trimmed[..5].join(" | "))
                            } else {
                                trimmed.join(" | ")
                            };
                            self.feedback_message = format!("[r] {}", text);
                        }
                        Err(e) => {
                            self.feedback_message = format!("[r] Error: {e}");
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.run_running = false;
                    self.run_rx = None;
                    self.feedback_message = "[r] Process disconnected".to_string();
                }
            }
        }
    }

    fn ensure_module_visible(&mut self, idx: usize) {
        if idx < self.scroll_offset {
            self.scroll_offset = idx;
        } else if self.visible_height > 0
            && idx >= self.scroll_offset + self.visible_height
        {
            self.scroll_offset = idx.saturating_sub(self.visible_height) + 1;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.check_loading();
        self.check_run();

        self.feedback_message.clear();

        if self.search_input.focused {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.search_input.focused = false;
                    let query = self.search_input.value.clone();
                    if !query.is_empty() {
                        self.search_input.clear();
                        if let Some(idx) = self.selected_category {
                            if idx < self.categories.len() {
                                let cat = self.categories[idx].clone();
                                self.load_modules(&cat);
                            }
                        }
                    }
                    return true;
                }
                KeyCode::Char(c) => {
                    self.search_input.insert_char(c);
                    self.show_detail = false;
                    self.apply_search_filter();
                    return true;
                }
                KeyCode::Backspace => {
                    self.search_input.delete_char();
                    if self.search_input.value.is_empty() {
                        if let Some(idx) = self.selected_category {
                            if idx < self.categories.len() {
                                let cat = self.categories[idx].clone();
                                self.load_modules(&cat);
                            }
                        }
                    } else {
                        self.apply_search_filter();
                    }
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

        if self.show_detail {
            match key.code {
                KeyCode::Up | KeyCode::Esc | KeyCode::Enter => {
                    self.show_detail = false;
                    self.full_info_content = None;
                    return true;
                }
                KeyCode::Char('r') => {
                    if let Some(idx) = self.selected_module {
                        if idx < self.displayed_modules.len() && !self.run_running {
                            let module_name = self.displayed_modules[idx].name.clone();
                            self.feedback_message = format!("[r] Running {}...", module_name);
                            self.run_running = true;
                            let (tx, rx) = mpsc::channel();
                            self.run_rx = Some(rx);
                            std::thread::spawn(move || {
                                let result = runner::run_msf_command(&[
                                    &format!("use {}", module_name),
                                    "run",
                                    "exit",
                                ]);
                                let _ = tx.send(result);
                            });
                        }
                    }
                    self.show_detail = false;
                    return true;
                }
                KeyCode::Char('i') => {
                    if let Some(idx) = self.selected_module {
                        if idx < self.displayed_modules.len() {
                            let module_name = &self.displayed_modules[idx].name;
                            match msfconsole::get_module_info(module_name) {
                                Ok(info) => {
                                    self.full_info_content = Some(info);
                                }
                                Err(e) => {
                                    self.feedback_message = format!("[i] Error: {e}");
                                }
                            }
                        }
                    }
                    return true;
                }
                _ => return true,
            }
        }

        match key.code {
            KeyCode::Char('/') => {
                self.search_input.focused = true;
                self.search_input.clear();
                self.show_detail = false;
                true
            }
            KeyCode::Up => {
                if self.selected_module.is_some() {
                    let idx = self.selected_module.unwrap();
                    if idx > 0 {
                        let new_idx = idx - 1;
                        self.selected_module = Some(new_idx);
                        self.ensure_module_visible(new_idx);
                    } else {
                        self.selected_module = None;
                    }
                } else {
                    let prev = self.selected_category.unwrap_or(0);
                    self.selected_category = Some(prev.saturating_sub(1));
                    let cat = self
                        .selected_category
                        .map(|i| self.categories[i].clone())
                        .unwrap_or_default();
                    self.load_modules(&cat);
                }
                true
            }
            KeyCode::Down => {
                if let Some(idx) = self.selected_module {
                    let next = idx + 1;
                    if next < self.displayed_modules.len() {
                        self.selected_module = Some(next);
                        self.ensure_module_visible(next);
                    }
                } else {
                    let next = self.selected_category.unwrap_or(0) + 1;
                    if next < self.categories.len() {
                        self.selected_category = Some(next);
                        let cat = self.categories[next].clone();
                        self.load_modules(&cat);
                    }
                }
                true
            }
            KeyCode::Right => {
                if self.selected_module.is_none() && !self.displayed_modules.is_empty() {
                    self.selected_module = Some(0);
                    self.scroll_offset = 0;
                } else if let Some(idx) = self.selected_module {
                    if idx + 1 < self.displayed_modules.len() {
                        let new_idx = idx + 1;
                        self.selected_module = Some(new_idx);
                        self.ensure_module_visible(new_idx);
                    }
                }
                true
            }
            KeyCode::Left => {
                if let Some(idx) = self.selected_module {
                    if idx > 0 {
                        let new_idx = idx - 1;
                        self.selected_module = Some(new_idx);
                        self.ensure_module_visible(new_idx);
                    } else {
                        self.selected_module = None;
                    }
                }
                true
            }
            KeyCode::Enter => {
                if self.selected_module.is_some() {
                    if self.show_detail {
                        self.full_info_content = None;
                    }
                    self.show_detail = !self.show_detail;
                }
                true
            }
            KeyCode::Esc => false,
            _ => false,
        }
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        _message: &str,
    ) -> String {
        self.check_loading();
        self.check_run();

        if !self.all_loaded
            && (self.feedback_message.is_empty()
                || self.feedback_message.starts_with("Loading modules..."))
        {
            let loaded =
                self.categories.len() - 1 - self.loading_categories.len();
            let total = self.categories.len() - 1;
            self.feedback_message =
                format!("Loading modules... ({loaded}/{total})");
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_search_bar(f, chunks[0]);

        if self.show_detail {
            if let Some(idx) = self.selected_module {
                if idx < self.displayed_modules.len() {
                    self.render_module_detail(
                        f,
                        chunks[1],
                        &self.displayed_modules[idx],
                    );
                    if !self.feedback_message.is_empty() {
                        return self.feedback_message.clone();
                    }
                    return format!("Viewing: {}", self.displayed_modules[idx].name);
                }
            }
        }

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(chunks[1]);

        self.render_category_list(f, main_chunks[0]);
        self.render_module_list(f, main_chunks[1]);

        if !self.feedback_message.is_empty() {
            return self.feedback_message.clone();
        }

        if let Some(idx) = self.selected_module {
            if idx < self.displayed_modules.len() {
                return format!("Selected: {}", self.displayed_modules[idx].name);
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

        let items: Vec<Line> = self
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

        f.render_widget(Paragraph::new(items).block(block), area);
    }

    fn render_module_list(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Modules ")
            .borders(Borders::ALL);

        if self.displayed_modules.is_empty() {
            let msg = if !self.all_loaded {
                Line::from(Span::styled(
                    " Loading modules... ",
                    Style::default().add_modifier(Modifier::DIM),
                ))
            } else {
                Line::from(Span::styled(
                    " No modules loaded.",
                    Style::default().add_modifier(Modifier::DIM),
                ))
            };
            f.render_widget(Paragraph::new(vec![msg]).block(block), area);
            return;
        }

        self.visible_height = (area.height.max(3) - 2) as usize;
        let total = self.displayed_modules.len();
        let offset = self.scroll_offset.min(total.saturating_sub(1));
        let end = (offset + self.visible_height).min(total);

        let items: Vec<Line> = self.displayed_modules[offset..end]
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let abs_idx = offset + i;
                let is_selected = self.selected_module == Some(abs_idx);
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

        f.render_widget(Paragraph::new(items).block(block), area);
    }

    fn render_module_detail(&self, f: &mut Frame, area: Rect, module: &MsfModule) {
        if let Some(ref info) = self.full_info_content {
            let lines: Vec<Line> = info
                .lines()
                .map(|line| Line::from(Span::raw(line)))
                .collect();
            let block = Block::default()
                .title(format!(" Full Info: {} ", module.name))
                .borders(Borders::ALL);
            f.render_widget(
                Paragraph::new(lines).block(block).wrap(Wrap { trim: false }),
                area,
            );
            return;
        }

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
            lines.push(Line::from(Span::raw(format!(
                " Disclosure:  {date}"
            ))));
        }

        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(Span::styled(
            " [Enter] Back to list  [r] Run module  [i] View info  ",
            Style::default().add_modifier(Modifier::DIM),
        )));

        let block = Block::default()
            .title(" Module Details ")
            .borders(Borders::ALL);

        f.render_widget(
            Paragraph::new(lines).block(block).wrap(Wrap { trim: false }),
            area,
        );
    }
}
