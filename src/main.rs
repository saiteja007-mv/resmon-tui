mod app;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let tick_rate = Duration::from_millis(500); // Update every 500ms
    let mut last_tick = Instant::now();

    // Run main loop
    let res = run_app(&mut terminal, &mut app, tick_rate, &mut last_tick);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
    last_tick: &mut Instant,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle input with timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        if !app.show_details {
                            app.quit();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.next_process();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.previous_process();
                    }
                    KeyCode::Enter => {
                        if app.selected_process.is_some() {
                            app.toggle_details();
                        }
                    }
                    KeyCode::Esc => {
                        if app.show_details {
                            app.toggle_details();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update data periodically
        if last_tick.elapsed() >= tick_rate {
            app.update();
            *last_tick = Instant::now();
        }

        // Check if should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
