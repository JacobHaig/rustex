#[allow(dead_code)]
mod app;

#[cfg(feature = "termion")]
mod events;
mod ui;

#[allow(dead_code)]
mod widgets;

// mod util;

// use crate::demo::ui;
// use crate::demo::App;

use argh::FromArgs;

use crossterm::event;
use crossterm::event::Event as CEvent;
use crossterm::event::KeyCode;

use crossterm::execute;
use crossterm::terminal;

use std::error::Error;
use std::io::stdout;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use tui::backend::CrosstermBackend;
use tui::Terminal;

enum Event<I> {
    InputEvent(I),
    Tick,
}

/// Crossterm demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (event_sender, event_receiver) = mpsc::channel();

    let tick_rate = Duration::from_millis(cli.tick_rate);

    // Spawn the input handling thread
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // poll for events
            if event::poll(timeout).unwrap() {
                let event = event::read().unwrap();
                let result = event_sender.send(Event::InputEvent(event));

                if result.is_err() {
                    break;
                }

                // if let CEvent::Key(key) =  event::read().unwrap() {
                //     event_sender.send(Event::Input(key)).unwrap();
                // }
            }

            // send tick event
            if last_tick.elapsed() >= tick_rate {
                event_sender.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut app = app::App::new("Crossterm Demo", cli.enhanced_graphics);

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match event_receiver.recv()? {
            Event::InputEvent(event) => match event {
                // Key Codes
                CEvent::Key(key) => match key.code {
                    // Handle closing the application with the escape key
                    KeyCode::Esc => {
                        terminal::disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            terminal::LeaveAlternateScreen,
                            event::DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        app.should_quit = true;
                    }
                    // handle all key events in app
                    _ => app.handle_keyboard_event(key),
                },
                CEvent::Mouse(_) => continue,
                CEvent::Resize(_, _) => continue,
                // KeyCode::Esc => {
                //     terminal::disable_raw_mode()?;
                //     execute!(
                //         terminal.backend_mut(),
                //         terminal::LeaveAlternateScreen,
                //         event::DisableMouseCapture
                //     )?;
                //     terminal.show_cursor()?;
                //     break;
                // }
                // _ => app.handle_keyboard_event(event.code),
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
