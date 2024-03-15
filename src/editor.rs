use crate::Terminal;
use std::io;
use termion::event::Key;

pub struct Editor {
    terminal: Terminal,
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
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
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_goto(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Exiting rvim.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_goto(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for i in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("{i}\r");
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
