mod app;
mod ui;

use anyhow::Result;
use app::{App, ProcessAction, SortOrder};
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
    let mut last_tick = Instant::now();

    // Run main loop
    let res = run_app(&mut terminal, &mut app, &mut last_tick);

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
    last_tick: &mut Instant,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle input with timeout
        let tick_rate = app.get_refresh_duration();
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Handle confirmation dialog first
                if app.pending_action.is_some() {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => {
                            app.execute_action();
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.cancel_action();
                        }
                        _ => {}
                    }
                }
                // Handle search mode separately
                else if app.search_mode {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.search_input(c);
                        }
                        KeyCode::Backspace => {
                            app.search_backspace();
                        }
                        KeyCode::Enter | KeyCode::Esc => {
                            app.exit_search();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('?') => {
                            app.toggle_help();
                        }
                        KeyCode::Char('/') => {
                            if !app.show_help {
                                app.start_search();
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            if !app.show_details && !app.show_help {
                                app.quit();
                            }
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            if !app.show_help {
                                app.increase_refresh_rate();
                            }
                        }
                        KeyCode::Char('-') => {
                            if !app.show_help {
                                app.decrease_refresh_rate();
                            }
                        }
                        KeyCode::Char('c') => {
                            if !app.show_help {
                                app.set_sort_order(SortOrder::Cpu);
                            }
                        }
                        KeyCode::Char('m') => {
                            if !app.show_help {
                                app.set_sort_order(SortOrder::Memory);
                            }
                        }
                        KeyCode::Char('p') => {
                            if !app.show_help {
                                app.set_sort_order(SortOrder::Pid);
                            }
                        }
                        KeyCode::Char('t') => {
                            if !app.show_help {
                                app.set_sort_order(SortOrder::Runtime);
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if !app.show_help {
                                app.next_process();
                            }
                        }
                        KeyCode::Up => {
                            if !app.show_help {
                                app.previous_process();
                            }
                        }
                        KeyCode::Char('k') => {
                            if !app.show_help && !app.show_details {
                                // 'k' for kill when in normal mode with process selected
                                if app.selected_process.is_some() {
                                    app.request_action(ProcessAction::Kill);
                                } else {
                                    // Otherwise use for navigation up
                                    app.previous_process();
                                }
                            } else if !app.show_help {
                                // In details view or help, just navigate
                                app.previous_process();
                            }
                        }
                        KeyCode::Char('s') => {
                            if !app.show_help && app.selected_process.is_some() && !app.show_details {
                                app.request_action(ProcessAction::Suspend);
                            }
                        }
                        KeyCode::Char('r') => {
                            if !app.show_help && app.selected_process.is_some() && !app.show_details {
                                app.request_action(ProcessAction::Resume);
                            }
                        }
                        KeyCode::Enter => {
                            if !app.show_help && app.selected_process.is_some() {
                                app.toggle_details();
                            }
                        }
                        KeyCode::Esc => {
                            if app.show_help {
                                app.toggle_help();
                            } else if app.show_details {
                                app.toggle_details();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update data periodically
        let tick_rate = app.get_refresh_duration();
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
