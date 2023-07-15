use crossterm::event::KeyCode;

use crate::widgets::navigation::NavigationWindow;
use crate::widgets::{file_manager::FileManagerWindow, Window};

#[derive(PartialEq)]
pub enum WindowState {
    Editor,
    FileMenu,
    Navigation,
}

pub struct Context {
    pub width: u16,
    pub height: u16,

    pub current_window: WindowState,

    pub file_manager_window: FileManagerWindow,
    pub navigation_window: NavigationWindow,
}

pub struct App {
    pub title: String,
    pub should_quit: bool,

    pub context: Context,
}

impl App {
    pub fn new(title: &str) -> App {
        App {
            title: title.to_string(),
            should_quit: false,

            context: Context {
                width: 0,
                height: 0,

                current_window: WindowState::Editor,

                file_manager_window: FileManagerWindow::new(),
                navigation_window: NavigationWindow::new(),
            },
            // interaction_menu_visable: false,
        }
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
        let keycode: KeyCode = event.code;
        let keyflags: u8 = event.modifiers.bits(); // ( BLANK, alt, control, shift) 0b0111

        // Catch all the keycodes and flags that match
        // And then change current window context

        // println!("Keycode: {:?} Keyflags: {:b}", keycode, keyflags);

        match (keycode, keyflags) {
            (KeyCode::Char('b'), 0b0010) => {
                if self.context.current_window != WindowState::Navigation {
                    self.context.current_window = WindowState::Navigation;
                } else {
                    self.context.current_window = WindowState::Editor;
                }
            }

            // Send the keypress to the current window
            _ => match self.context.current_window {
                WindowState::Editor => {
                    self.context
                        .file_manager_window
                        .handle_keyboard_input(keycode, keyflags);
                }
                WindowState::Navigation => {
                    self.context
                        .navigation_window
                        .handle_keyboard_input(keycode, keyflags);
                }
                WindowState::FileMenu => {}
            },
        }

        // dbg!(flags);

        // if !file_edit_mode {
        //     match keycode {
        //         KeyCode::Char('`') => self.toggle_interaction_menu(),
        //         KeyCode::Enter => self.interaction_menu.run(),

        //         KeyCode::Left => self.file_menu_state.previous(),
        //         KeyCode::Right => self.file_menu_state.next(),

        //         KeyCode::Up => self.interaction_menu.previous(),
        //         KeyCode::Down => self.interaction_menu.next(),
        //         _ => {}
        //     }
        // } else if file_edit_mode {
        //     match flags {
        //         // Holding Alt
        //         // Note, when run in an IDE the alt key may be eaten by the IDE
        //         0b0100 => match keycode {
        //             KeyCode::Down => todo!("Move Current Line DOWN"),
        //             KeyCode::Up => todo!("Move Current Line UP"),
        //             _ => {}
        //         },

        //         // No modifiers, holding Shift, unknown combination
        //         0b0000 | _ => match keycode {
        //             KeyCode::Char('`') => self.toggle_interaction_menu(),

        //             KeyCode::Char(c) => self.file_menu_state.current_file_menu().insert_char(c),
        //             KeyCode::Enter => self.file_menu_state.current_file_menu().insert_new_line(),
        //             KeyCode::Backspace => self.file_menu_state.current_file_menu().backspace_char(),
        //             KeyCode::Delete => self.file_menu_state.current_file_menu().delete_char(),

        //             KeyCode::Left => self.file_menu_state.current_file_menu().move_cursor_x(-1),
        //             KeyCode::Right => self.file_menu_state.current_file_menu().move_cursor_x(1),
        //             KeyCode::Up => self.file_menu_state.current_file_menu().move_cursor_y(-1),
        //             KeyCode::Down => self.file_menu_state.current_file_menu().move_cursor_y(1),
        //             _ => {}
        //         },
        //     }
        // }
    }
}
