mod app;
mod event;
mod game;
mod ui;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use crate::app::App;
use crate::event::{Event, EventHandler};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Random seed for deterministic gameplay
    #[arg(short, long)]
    seed: Option<u64>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(cli.seed);
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<()> {
    let events = EventHandler::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        match events.next()? {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Esc => {
                        if key.modifiers.contains(KeyModifiers::NONE) {
                            app.should_quit = true;
                        }
                    }
                    KeyCode::Char('c') => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            app.should_quit = true;
                        } else {
                            app.handle_input('c');
                        }
                    }
                    KeyCode::Enter => {
                        app.handle_enter()?;
                    }
                    KeyCode::Backspace => {
                        app.handle_backspace();
                    }
                    KeyCode::Char(c) => {
                        app.handle_input(c);
                    }
                    _ => {}
                }
            }
            Event::Tick => {
                app.check_splash_timeout();
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}