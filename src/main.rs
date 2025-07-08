use crossterm::{
    ExecutableCommand,
    event::{self as crossterm_event, Event}, // Alias event to avoid conflict with our own `event` module
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io::{self, stdout},
    time::Duration,
};

// Import everything we need directly from your library (src/lib.rs)
use freewrite_tui::event;
use freewrite_tui::ui;
use freewrite_tui::{App, AppState};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    let tick_rate = Duration::from_millis(100);
    let last_tick = std::time::Instant::now();

    loop {
        // Draw UI
        terminal.draw(|frame| ui::ui(frame, &app))?;

        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm_event::poll(timeout)? {
            // Use aliased crossterm_event
            if let Event::Key(key_event) = crossterm_event::read()? {
                event::handle_event(
                    &mut app,
                    key_event.kind,
                    key_event.code,
                    key_event.modifiers,
                )?;
            }
        }

        // Update application state
        app.update();

        // Check if quitting
        if let AppState::Quitting = app.current_state {
            break;
        }
    }

    // Restore terminal
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
