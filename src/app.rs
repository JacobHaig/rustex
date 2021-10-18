use crossterm::event::KeyCode;

use crate::widgets::{FileMenuState, StatefulList};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,

    pub files: FileMenuState,

    pub interaction_menu_visable: bool,
    pub interaction_menu: StatefulList,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            files: FileMenuState::new(),

            interaction_menu: StatefulList::new(),
            interaction_menu_visable: false,
        }
    }

    pub fn on_up(&mut self) {
        self.interaction_menu.previous();
    }

    pub fn on_down(&mut self) {
        self.interaction_menu.next();
    }

    pub fn on_right(&mut self) {
        self.files.next();
    }

    pub fn on_left(&mut self) {
        self.files.previous();
    }

    pub fn toggle_interaction_menu(&mut self) {
        self.interaction_menu_visable = !self.interaction_menu_visable;

        // Reset the state of the menu ie the selected item
        self.interaction_menu.state.select(None);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        // self.progress += 0.001;
        // if self.progress > 1.0 {
        //     self.progress = 0.0;
        // }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }

    // handle_keyboard_event is the main function for sorting
    // out what happens when a key is pressed.
    pub fn handle_keyboard_event(&mut self, event: crossterm::event::KeyEvent) {
        let file_edit_mode = !self.interaction_menu_visable; // I might want to implement a FSM for this

        let code = event.code;
        let flags = event.modifiers.bits(); // ( alt, control, shift) 0b0111

        if self.interaction_menu_visable {
            match code {
                KeyCode::Char('`') => self.toggle_interaction_menu(),
                KeyCode::Enter => self.interaction_menu.run(),

                KeyCode::Left => self.on_left(),
                KeyCode::Up => self.on_up(),
                KeyCode::Right => self.on_right(),
                KeyCode::Down => self.on_down(),
                _ => {}
            }
        } else if file_edit_mode {
            dbg!("{:?}", flags);

            match flags {
                // Holding Alt
                // Note, when run in an IDE the alt key may not work
                0b0100 => match code {
                    KeyCode::Down => todo!("Move Current Line DOWN"),
                    KeyCode::Up => todo!("Move Current Line UP"),
                    _ => {}
                },

                // No modifiers, holding Shift, unknown combination
                0b0000 | _ => match code {
                    KeyCode::Char('`') => self.toggle_interaction_menu(),

                    KeyCode::Char(c) => self.files.current_file_menu().insert_char(c),
                    KeyCode::Enter => self.files.current_file_menu().insert_new_line(),
                    KeyCode::Backspace => self.files.current_file_menu().backspace_char(),
                    KeyCode::Delete => self.files.current_file_menu().delete_char(),

                    KeyCode::Left => self.files.current_file_menu().move_cursor_x(-1),
                    KeyCode::Right => self.files.current_file_menu().move_cursor_x(1),
                    KeyCode::Up => self.files.current_file_menu().move_cursor_y(-1),
                    KeyCode::Down => self.files.current_file_menu().move_cursor_y(1),
                    _ => {}
                },
            }
        }
    }
}
