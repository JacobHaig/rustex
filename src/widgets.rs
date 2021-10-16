use std::{cmp::min, ffi::OsString, io::Write, path::Path};

use tui::widgets::ListState;
// #[cfg(feature = "termion")]
// pub mod event;

// use rand::distributions::{Distribution, Uniform};
// use rand::rngs::ThreadRng;

// #[derive(Clone)]
// pub struct RandomSignal {
//     distribution: Uniform<u64>,
//     rng: ThreadRng,
// }

// impl RandomSignal {
//     pub fn new(lower: u64, upper: u64) -> RandomSignal {
//         RandomSignal {
//             distribution: Uniform::new(lower, upper),
//             rng: rand::thread_rng(),
//         }
//     }
// }

// impl Iterator for RandomSignal {
//     type Item = u64;
//     fn next(&mut self) -> Option<u64> {
//         Some(self.distribution.sample(&mut self.rng))
//     }
// }

// #[derive(Clone)]
// pub struct SinSignal {
//     x: f64,
//     interval: f64,
//     period: f64,
//     scale: f64,
// }

// impl SinSignal {
//     pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
//         SinSignal {
//             x: 0.0,
//             interval,
//             period,
//             scale,
//         }
//     }
// }

// impl Iterator for SinSignal {
//     type Item = (f64, f64);
//     fn next(&mut self) -> Option<Self::Item> {
//         let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
//         self.x += self.interval;
//         Some(point)
//     }
// }

pub struct FileMenuStates {
    pub tab_index: usize,
    pub file_list: Vec<FileMenu>,
}

impl FileMenuStates {
    pub fn new() -> FileMenuStates {
        FileMenuStates {
            tab_index: 0,
            file_list: vec![FileMenu::new("HI"), FileMenu::new("Hellooo!")],
        }
    }

    pub fn add_file_menu(&mut self, file_menu: FileMenu) {
        self.file_list.push(file_menu);
    }

    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.file_list.len();
    }

    pub fn previous(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.file_list.len() - 1;
        }
    }

    pub fn current_file_menu(&mut self) -> &mut FileMenu {
        &mut self.file_list[self.tab_index]
    }
}

#[derive(Clone, Default, Debug)]
pub struct FileMenu {
    pub name: OsString,
    pub path: Option<Box<Path>>,

    pub cursor_x: usize,
    pub cursor_y: usize,

    pub selection_start_x: usize,
    pub selection_start_y: usize,
    pub selection_end_x: usize,
    pub selection_end_y: usize,
    pub selection_active: bool,

    pub lines: Vec<String>,
    pub scroll_offset: usize,
}

impl FileMenu {
    pub fn new(name: &str) -> FileMenu {
        FileMenu {
            name: OsString::from(name),
            path: None,
            lines: vec!["".to_string()],
            ..Default::default()
        }
    }
    pub fn open_file(path: Box<Path>) -> Result<FileMenu, std::io::Error> {
        let name = path.file_name().unwrap().to_os_string();

        let lines = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();

        let path = Some(path);

        Ok(FileMenu {
            name,
            path,
            lines,
            ..Default::default()
        })
    }

    pub fn save_file(&self) -> Result<(), std::io::Error> {
        let path = self.path.clone().unwrap();

        let mut file = std::fs::File::create(path)?;
        file.write_all(self.lines.join("\n").as_bytes())?;

        Ok(())
    }

    pub fn save_as_file(&mut self, path: Box<Path>) -> Result<(), std::io::Error> {
        self.name = path.file_name().unwrap().to_os_string();
        self.path = Some(path);

        self.save_file()
    }

    /// This can be used to add a new line to the file.
    pub fn insert_line(&mut self, line: String) {
        self.lines.insert(self.cursor_y + 1, line);
        self.move_cursor_y(1);
        self.reset_cursor();
    }

    /// This can be used to add a new line at the cursor position.
    pub fn insert_new_line(&mut self) {
        let (one, two) = self.lines[self.cursor_y].split_at(self.cursor_x);
        let one = one.to_string();
        let two = two.to_string();

        self.lines.insert(self.cursor_y + 1, String::new());

        self.lines[self.cursor_y] = one;
        self.lines[self.cursor_y + 1] = two;

        self.move_cursor_y(1);
        self.cursor_x = 0;
        self.reset_cursor();
    }

    pub fn move_cursor_x(&mut self, dx: isize) {
        if self.cursor_x as isize + dx >= 0 {
            self.cursor_x = (self.cursor_x as isize + dx) as usize;
        }
        self.reset_cursor();
    }

    /// move_cursor_y(-1) moves the cursor up one line.
    /// move_cursor_y(1) moves the cursor down one line.
    pub fn move_cursor_y(&mut self, dy: isize) {
        if self.cursor_y as isize + dy >= 0 {
            self.cursor_y = (self.cursor_y as isize + dy) as usize;
        }
        self.reset_cursor();
    }
    // reset_cursor fixes the cursor position so that
    // the position is always valid.
    // This is usefull for new lines, line deletions, and removing characters.
    // In this case, we simply reset the cursor anytime we move the cursor.
    pub fn reset_cursor(&mut self) {
        // self.cursor_x = 0;
        // self.cursor_y = 0;

        self.cursor_y = min(self.cursor_y, self.lines.len() - 1);
        self.cursor_x = min(self.cursor_x, self.lines[self.cursor_y].len());
    }

    pub fn insert_char(&mut self, c: char) {
        let mut line = self.lines[self.cursor_y].clone();

        // println!("self.cursor_x {}  -- insert_char: {}", self.cursor_x, c);
        line.insert(self.cursor_x, c);
        self.lines[self.cursor_y] = line;

        self.reset_cursor();
        self.move_cursor_x(1);
    }

    // delete_line removes a line from the file.
    // If the line has text to the right of the cursor,
    // the text is moved to the above line.
    fn backspace_line(&mut self) {
        if self.lines.len() > 1 {
            // let x1 = self.lines[self.cursor_y - 1].len();

            let new_y = self.cursor_y - 1;
            let new_x = self.lines[self.cursor_y - 1].len();

            let current_line = self.lines[self.cursor_y].clone();
            self.lines[self.cursor_y - 1] += current_line.as_str();

            self.cursor_y = new_y;
            self.cursor_x = new_x;
            self.reset_cursor();

            self.lines.remove(self.cursor_y + 1);
        }
    }

    fn delete_line(&mut self) {
        if self.lines.len() > self.cursor_y + 1 {
            let current_line = self.lines[self.cursor_y + 1].clone();
            self.lines[self.cursor_y] += current_line.as_str();

            self.reset_cursor();

            self.lines.remove(self.cursor_y + 1);
        }
    }

    pub fn delete_char(&mut self) {
        let mut line = self.lines[self.cursor_y].clone();

        // If the cursor is at the end of the line,
        // we need to delete the new line.
        if self.cursor_x == line.len() {
            self.delete_line();
        } else {
            // If the Cursur is anywhere else, we just remove the character.
            line.remove(self.cursor_x);
            self.lines[self.cursor_y] = line;

            // self.move_cursor_x(-1);
            self.reset_cursor();
        }
    }

    pub fn backspace_char(&mut self) {
        let mut line = self.lines[self.cursor_y].clone();

        // If the cursor is at the beginning of the line,
        // we need to delete the new line.
        if self.cursor_x == 0 {
            self.backspace_line();
        } else {
            // If the Cursur is anywhere else, we just remove the character.
            line.remove(self.cursor_x - 1);
            self.lines[self.cursor_y] = line;

            self.move_cursor_x(-1);
            self.reset_cursor();
        }
    }

    // return a slice of the lines in the file given a start and end line.
    pub fn get_lines(&self, start: usize, mut end: usize) -> &[String] {
        // Verify that the start and end are valid.
        // assert!(start <= end);

        end = min(self.lines.len(), end);

        &self.lines[start..end]
    }

    // Given a line offset, return the lines that are visible.
    pub fn get_visible_lines(&self, line_amount: usize) -> &[String] {
        let start = self.scroll_offset;
        let end = start + line_amount;
        self.get_lines(start, end)
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
