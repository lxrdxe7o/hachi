#![allow(dead_code)]
mod  app;
mod daemon;
mod error;
mod ui;

#[cfg(test)]
mod tests;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;
use crate::daemon::DaemonHandle;

/// Target frame rate
const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Hide cursor
    terminal.hide_cursor()?;

    // Spawn hardware actor
    let daemon = DaemonHandle::spawn();

    // Request initial state
    daemon.refresh();

    // Create application (daemon ownership transferred)
    let mut app = App::new(daemon);

    // Initialize sakura particles with terminal size
    let size = terminal.size()?;
    app.init_sakura(size.width, size.height);

    // Run the main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Shutdown hardware actor (app owns daemon)
    app.shutdown();

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        // Process hardware updates
        app.process_updates();

        // Update timing and effects
        app.tick();

        // Render
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        // Handle input with timeout for smooth animation
        if event::poll(FRAME_DURATION)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    app.handle_key(key);
                }
                Event::Resize(width, height) => {
                    app.resize(width, height);
                }
                _ => {}
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
