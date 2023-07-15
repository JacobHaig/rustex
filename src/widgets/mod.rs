use crossterm::event::KeyCode;

pub mod file;
pub mod file_manager;
pub mod navigation;

pub trait Window {
    fn handle_keyboard_input(&mut self, keycode: KeyCode, keyflags: u8);
    // fn draw(&mut self, frame: &mut tui::Frame<impl tui::backend::Backend>, area: tui::layout::Rect);
}
