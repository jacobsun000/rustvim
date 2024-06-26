use crate::Pos;
use std::io::{self, stdin, stdout, Write};
use termion::{self, color, event::Key, input::TermRead, raw::IntoRawMode};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: termion::raw::RawTerminal<io::Stdout>,
}

pub enum CursorShape {
    BlinkingBar,
    BlinkingBlock,
    BlinkingUnderline,
    SteadyBar,
    SteadyBlock,
    SteadyUnderline,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1 - 2,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_goto(pos: &Pos) {
        let x = (pos.x + 1) as u16;
        let y = (pos.y + 1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
    
    pub fn set_cursor_shape(shape: CursorShape) {
        match shape {
            CursorShape::BlinkingBar => print!("{}", termion::cursor::BlinkingBar),
            CursorShape::BlinkingBlock => print!("{}", termion::cursor::BlinkingBlock),
            CursorShape::BlinkingUnderline => print!("{}", termion::cursor::BlinkingUnderline),
            CursorShape::SteadyBar => print!("{}", termion::cursor::SteadyBar),
            CursorShape::SteadyBlock => print!("{}", termion::cursor::SteadyBlock),
            CursorShape::SteadyUnderline => print!("{}", termion::cursor::SteadyUnderline),
        }
    }
}
