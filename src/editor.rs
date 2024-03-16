use crate::Document;
use crate::Row;
use crate::Terminal;
use std::cmp::{max, min};
use std::{env, io};
use termion::color;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

#[derive(Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    cursor_pos: Pos,
    offset: Pos,
    document: Document,
    terminal: Terminal,
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            cursor_pos: Pos::default(),
            offset: Pos::default(),
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
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

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
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
        self.scroll();
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_goto(&Pos::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Exiting rvim.\r");
        } else {
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
        let mut filename = self
            .document
            .filename
            .clone()
            .unwrap_or("[No Name]".to_string());
        filename.truncate(20);
        let file_status = format!("{} - {} lines", filename, self.document.len());
        let line_indicator = format!("{}:{}", self.cursor_pos.x + 1, self.cursor_pos.y + 1);
        let mut status = format!("{} {}", file_status, line_indicator);
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
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Pos { mut x, mut y } = self.cursor_pos;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = max(y - 1, 0),
            Key::Down => y = min(y + 1, height),
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
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
            Key::PageUp => y = max(y - terminal_height, 0),
            Key::PageDown => y = min(y + terminal_height, height),
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
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
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
