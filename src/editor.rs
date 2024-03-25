use crate::{Document, Mode, Row, Terminal};
use std::cmp::{max, min};
use std::time::{Duration, Instant};
use std::{env, io};
use termion::color;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

#[derive(Default, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    cursor_pos: Pos,
    offset: Pos,
    mode: Mode,
    document: Document,
    terminal: Terminal,
    should_quit: bool,
    status_message: StatusMessage,
    highlighted_word: Option<String>,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: <C-S> = save <C-Q> = quit <C-F> = search");
        let document = if let Some(filename) = args.get(1) {
            if let Ok(doc) = Document::open(filename) {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", filename);
                Document::default()
            }
        } else {
            Document::default()
        };
        Terminal::set_cursor_shape(Mode::Normal.cursor_shape());

        Self {
            cursor_pos: Pos::default(),
            offset: Pos::default(),
            should_quit: false,
            mode: Mode::Normal,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            document,
            status_message: StatusMessage::from(initial_status),
            highlighted_word: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.handle_input() {
                die(&error);
            }
        }
    }

    fn handle_input(&mut self) -> Result<(), io::Error> {
        match self.mode {
            Mode::Normal => self.handle_normal_mode_input()?,
            Mode::Insert => self.handle_insert_mode_input()?,
            Mode::Visual => self.handle_visual_mode_input()?,
            Mode::Command => self.handle_command_mode_input()?,
        }
        self.scroll();
        Ok(())
    }

    fn handle_normal_mode_input(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Esc => (),
            Key::Char('i') => self.set_mode(Mode::Insert),
            Key::Char('v') => self.set_mode(Mode::Visual),
            Key::Char(':') => self.set_mode(Mode::Command),
            Key::Ctrl('q') => self.quit(),
            Key::Ctrl('x') => self.should_quit = true,
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('f') => self.search(),
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn handle_insert_mode_input(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Esc => self.set_mode(Mode::Normal),
            Key::Char(c) => self.insert(c),
            Key::Delete => self.delete(Direction::Forward),
            Key::Backspace => self.delete(Direction::Backward),
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn handle_visual_mode_input(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Esc => self.set_mode(Mode::Normal),
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn handle_command_mode_input(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Esc => self.set_mode(Mode::Normal),
            _ => (),
        }
        Ok(())
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        Terminal::set_cursor_shape(mode.cursor_shape())
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_goto(&Pos::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Exiting rvim.\r");
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(self.offset.y + self.terminal.size().height as usize),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_goto(&Pos {
                x: self.cursor_pos.x - self.offset.x,
                y: self.cursor_pos.y - self.offset.y,
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row)
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("RVim editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.size().width as usize;
        let mode = format!("[{}]", self.mode.name());
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        let mut filename = self
            .document
            .file_name
            .clone()
            .unwrap_or("[No Name]".to_string());
        filename.truncate(20);
        let file_status = format!(
            "{} - {} lines{}",
            filename,
            self.document.len(),
            modified_indicator
        );
        let line_indicator = format!(
            "{} | {}:{}",
            self.document.file_type(),
            self.cursor_pos.x + 1,
            self.cursor_pos.y + 1
        );
        let mut status = format!("{mode} {file_status} {line_indicator}");
        status = format!("{:width$}", status, width = width);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Pos { mut x, mut y } = self.cursor_pos;
        let height = self.document.len();
        let mut width = self.document.row(y).map(|r| r.len()).unwrap_or(0);
        match key {
            Key::Up => y = max(y.saturating_sub(1), 0),
            Key::Down => y = min(y + 1, height),
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.document.row(y).map(|r| r.len()).unwrap_or(0);
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => y = max(y.saturating_sub(terminal_height), 0),
            Key::PageDown => y = min(y + terminal_height, height),
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = self.document.row(y).map(|r| r.len()).unwrap_or(0);
        x = min(x, width);
        self.cursor_pos = Pos { x, y };
    }

    fn scroll(&mut self) {
        let Pos { x, y } = self.cursor_pos;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x + width {
            self.offset.x = x - width + 1
        }

        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y + height {
            self.offset.y = y - height + 1
        }
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result);
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);

            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            } else {
                self.document.file_name = new_name;
            }
        }
        let message = if self.document.save().is_ok() {
            "File saved successfully"
        } else {
            "Error writing to file!"
        };
        self.status_message = StatusMessage::from(message.to_string());
    }

    fn search(&mut self) {
        let old_pos = self.cursor_pos.clone();
        let mut direction = Direction::Forward;
        let query = self
            .prompt(
                "Search (ESC to caecel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            direction = Direction::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        }
                        Key::Left | Key::Up => direction = Direction::Backward,
                        _ => direction = Direction::Forward,
                    }
                    if let Some(pos) = editor.document.find(query, &editor.cursor_pos, direction) {
                        editor.cursor_pos = pos;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                    editor.highlighted_word = Some(query.to_string());
                },
            )
            .unwrap_or(None);
        if query.is_none() {
            self.cursor_pos = old_pos;
            self.scroll();
        }
        self.highlighted_word = None;
    }

    fn quit(&mut self) {
        if self.document.is_dirty() {
            self.status_message = StatusMessage::from(
                "WARNING! File has unsaved changes. Please use <C-X> to abort changes".to_string(),
            );
        } else {
            self.should_quit = true;
        }
    }

    fn insert(&mut self, c: char) {
        self.document.insert(&self.cursor_pos, c);
        self.move_cursor(Key::Right)
    }

    fn delete(&mut self, direction: Direction) {
        match direction {
            Direction::Backward => {
                if self.cursor_pos.x > 0 || self.cursor_pos.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_pos);
                }
            }
            Direction::Forward => {
                self.document.delete(&self.cursor_pos);
            }
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
