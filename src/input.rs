use crate::InputEvent;

use crossterm::event;
use crossterm::event::Event;

use std::sync::mpsc;

pub fn input_handler(
    tick_rate: std::time::Duration,
    event_sender: mpsc::Sender<InputEvent<Event>>,
) {
    let mut last_tick = std::time::Instant::now();
    loop {
        // poll for tick rate duration, if no events, sent tick event.
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(1));

        // poll for events
        if event::poll(timeout).unwrap() {
            let event = event::read().unwrap();
            let result = event_sender.send(InputEvent::InputEvent(event));

            if result.is_err() {
                break;
            }
        }

        // send tick event
        if last_tick.elapsed() >= tick_rate {
            event_sender.send(InputEvent::Tick).unwrap();
            last_tick = std::time::Instant::now();
        }
    }
}
