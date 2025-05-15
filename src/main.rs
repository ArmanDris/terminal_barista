mod app;
mod cup;
mod liquids;
mod ui;

use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use cup::{pour_a_into_b, scramble_cups};

use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};

fn _main_helper() -> Result<(), String> {
    let mut r = scramble_cups();
    if let Some(cup) = r.last() {
        println!("Last cup: {:?}", cup);
    }
    r.pop();
    if let Some(cup) = r.last() {
        println!("Last cup: {:?}", cup);
    }
    r.pop();
    if let Some(cup) = r.last() {
        println!("Last cup: {:?}", cup);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip key release events
                continue;
            }
            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        app.tooltip = None;
                        if app.src_selection.is_none() {
                            let raw_src_sel = c.to_digit(10);
                            if matches!(raw_src_sel, None | Some(0)) {
                                continue;
                            }
                            app.src_selection = Some(raw_src_sel.unwrap() - 1);
                        } else {
                            let raw_dst_sel = c.to_digit(10);
                            if matches!(raw_dst_sel, None | Some(0)) {
                                continue;
                            }
                            let dst_sel = (raw_dst_sel.unwrap() - 1) as usize;
                            let src_sel = app.src_selection.unwrap() as usize;
                            app.src_selection = None;
                            if src_sel >= app.cups.len() || dst_sel >= app.cups.len() {
                                continue;
                            }
                            let r = pour_a_into_b(&app.cups[src_sel], &app.cups[dst_sel]);
                            match r {
                                Ok((new_src, new_dst)) => {
                                    app.cups[src_sel] = new_src;
                                    app.cups[dst_sel] = new_dst;
                                }
                                Err(msg) => app.tooltip = Some(msg),
                            }
                            if cup::are_cups_solved(&app.cups) {
                                app.tooltip = Some("You win!".to_string());
                                app.current_screen = CurrentScreen::Finished;
                            }
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Finished => {
                    if key.code == KeyCode::Enter {
                        *app = App::new();
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if res.is_ok() {
        println!("App exited with Ok");
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
