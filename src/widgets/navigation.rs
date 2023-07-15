use crossterm::event::KeyCode;
use tui::widgets::ListState;

use strum::*;
use strum_macros::Display;
use strum_macros::*;

use super::Window;

#[derive(Debug, Display, Clone, EnumString, EnumIter, EnumCount, PartialEq, Eq, Hash)]
#[strum(serialize_all = "title_case")]
pub enum ActionMenuWindow {
    // Example of how to set a specific name.
    // #[strum(serialize = "New File")]
    NewFile,
    OpenFile,
    SaveFile,
    SaveFileAs,
    SaveAll,
    CloseFile,
}

pub struct NavigationWindow {
    pub list_state: tui::widgets::ListState,
    // pub items: MenuAction, // This is currently not important.
}

impl Window for NavigationWindow {
    fn handle_keyboard_input(&mut self, keycode: crossterm::event::KeyCode, _keyflags: u8) {
        match keycode {
            // KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Enter => self.run(),
            // KeyCode::Backspace => self.backspace_char(),
            KeyCode::Delete => self.unselect(),

            // KeyCode::Left => self.move_cursor_x(-1),
            // KeyCode::Right => self.move_cursor_x(1),
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),

            _ => (),
        }
    }
}

impl NavigationWindow {
    pub fn new() -> NavigationWindow {
        NavigationWindow {
            list_state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        let new_index = if let Some(index) = self.list_state.selected() {
            index + 1
        } else {
            0
        };

        let wrapped_index = Some((new_index).rem_euclid(ActionMenuWindow::COUNT));

        self.list_state.select(wrapped_index);
    }

    pub fn previous(&mut self) {
        let new_index = if let Some(index) = self.list_state.selected() {
            index as isize - 1
        } else {
            0
        };

        let wrapped_index =
            Some((new_index as isize).rem_euclid(ActionMenuWindow::COUNT as isize) as usize);

        self.list_state.select(wrapped_index);
    }

    pub fn unselect(&mut self) {
        self.list_state.select(None);
    }

    // This function preforms the action that is selected.
    pub fn run(&self) {
        if let Some(i) = self.list_state.selected() {
            match ActionMenuWindow::iter().nth(i).unwrap() {
                ActionMenuWindow::NewFile => println!("New File"),
                ActionMenuWindow::OpenFile => println!("Open File"),
                ActionMenuWindow::SaveFile => println!("Save File"),
                ActionMenuWindow::SaveFileAs => println!("Save File As"),
                ActionMenuWindow::SaveAll => println!("Save All"),
                ActionMenuWindow::CloseFile => println!("Close File"),
            }
        }
    }
}
