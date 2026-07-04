#![allow(dead_code)]

use std::io::stdout;
use std::panic;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod app;
mod components;
mod msf;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = app::App::new();
    let result = app.run(&mut terminal);

    restore_terminal()?;
    result
}

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
