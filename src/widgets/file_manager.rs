use crossterm::event::KeyCode;

use crate::widgets::file::FileWindow;
use crate::widgets::Window;

pub struct FileManagerWindow {
    pub current_file_index: usize,
    pub file_list: Vec<FileWindow>,
}

impl Window for FileManagerWindow {
    // keyflags = (BLANK, alt, control, shift) 0b0111
    fn handle_keyboard_input(&mut self, keycode: KeyCode, keyflags: u8) {
        match (keycode, keyflags) {
            (KeyCode::Char('n'), 0b0010) => self.next(),
            (KeyCode::Char('p'), 0b0010) => self.previous(),

            _ => self.current_file().handle_keyboard_input(keycode, keyflags),
        }
    }
}

impl FileManagerWindow {
    pub fn new() -> FileManagerWindow {
        FileManagerWindow {
            current_file_index: 0,
            file_list: vec![FileWindow::new("HI"), FileWindow::new("Hellooo!")],
        }
    }

    pub fn add_file_menu(&mut self, file: FileWindow) {
        self.file_list.push(file);
    }

    pub fn next(&mut self) {
        self.current_file_index = (self.current_file_index + 1) % self.file_list.len();
    }

    pub fn previous(&mut self) {
        if self.current_file_index > 0 {
            self.current_file_index -= 1;
        } else {
            self.current_file_index = self.file_list.len() - 1;
        }
    }

    pub fn current_file(&mut self) -> &mut FileWindow {
        &mut self.file_list[self.current_file_index]
    }
}
