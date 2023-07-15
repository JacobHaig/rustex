#[cfg(feature = "termion")]
mod events;
mod ui;

#[allow(dead_code)]
mod app;

mod widgets;

use argh::FromArgs;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;

use crossterm::terminal;

use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use tui::backend::CrosstermBackend;
use tui::Terminal;

mod input;
#[derive(Debug)]
pub enum InputEvent<T> {
    InputEvent(T),
    Tick,
}

/// Crossterm demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    // whether unicode symbols are used to improve the overall look of the app
    // #[argh(option, default = "true")]
    // enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::App::new("Rustex");
    let cli: Cli = argh::from_env();

    terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    crossterm::execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Input handling is done through a channel.
    let (event_sender, event_receiver) = mpsc::channel();

    // Spawn the input handling thread
    let tick_rate = std::time::Duration::from_millis(cli.tick_rate);
    std::thread::spawn(move || {
        input::input_handler(tick_rate, event_sender);
    });

    loop {
        handle_event(&event_receiver, &mut terminal, &mut app);

        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

pub fn handle_event(
    event_receiver: &Receiver<InputEvent<Event>>,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut app::App,
) {
    let event_received = event_receiver.recv().unwrap();

    match event_received {
        InputEvent::InputEvent(input_event) => match input_event {
            Event::Key(key_event) => match key_event.code {
                // Handle closing the application with the escape key
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();

                    crossterm::execute!(
                        terminal.backend_mut(),
                        terminal::LeaveAlternateScreen,
                        event::DisableMouseCapture
                    )
                    .unwrap();

                    terminal.show_cursor().unwrap();
                    app.should_quit = true;
                }

                // handle all key events in app
                _ => {
                    // Only handle key events if it is pressed down
                    if key_event.kind == event::KeyEventKind::Press {
                        app.handle_keyboard_event(key_event);
                    }
                }
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Paste(_) => {}
        },
        InputEvent::Tick => {
            app.on_tick();
        }
    }
}
