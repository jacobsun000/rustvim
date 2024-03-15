use crate::Document;
use crate::Row;
use crate::Terminal;
use std::{env, io};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    cursor_pos: Pos,
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
            Terminal::cursor_goto(&self.cursor_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
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
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn move_cursor(&mut self, key: Key) {
        let Pos { mut x, mut y } = self.cursor_pos;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1).max(0),
            Key::Down => y = y.saturating_add(1).min(height),
            Key::Left => x = x.saturating_sub(1).max(0),
            Key::Right => x = x.saturating_add(1).min(width),
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            _ => (),
        }
        self.cursor_pos = Pos { x, y };
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
