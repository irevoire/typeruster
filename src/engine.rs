use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};

pub struct Engine {
    text: Vec<char>,
    position: usize,
    // if an error was made, then error store the position were you fucked up
    error: Option<usize>,
    stdin: termion::input::Keys<std::io::Stdin>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

pub enum State {
    Stopped,
    Running,
    Finished(u32),
}

pub use State::*;

impl Engine {
    pub fn new(text: &str) -> Self {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        print!("{}", text); // TODO save the position of the cursor before
                            // printing the text so we can restore it right after
        print!("{}", cursor::Left(200));
        stdout.flush().unwrap();

        Engine {
            text: text.chars().collect(),
            position: 0,
            error: None,
            stdin: stdin.keys(),
            stdout,
        }
    }

    pub fn cycle(&mut self) -> State {
        let k = match self.stdin.next() {
            None => return Stopped,
            Some(o) => o,
        };

        match k.unwrap() {
            Key::Ctrl('c') => return Stopped,
            Key::Esc => return Stopped,

            Key::Backspace => self.handle_backspace(),
            Key::Char(c) => self.handle_keys(c),
            a => println!("\nUnexpected argument {:?}", a),
        }

        self.stdout.flush().unwrap();
        if self.position == self.text.len() && self.error.is_none() {
            self.stdout.suspend_raw_mode().unwrap();
            println!("{}", color::Fg(color::Reset));
            Finished(2)
        } else {
            Running
        }
    }

    fn handle_keys(&mut self, k: char) {
        let next_key = self.text.iter().nth(self.position);
        if next_key.is_none() {
            return;
        }
        let next_key = *next_key.unwrap();
        if next_key == k && self.error.is_none() {
            print!("{}{}", color::Fg(color::Green), next_key);
        } else if next_key == k && self.error.is_some() {
            print!("{}{}", color::Fg(color::Reset), next_key);
        } else {
            self.error = Some(self.position);
            print!("{}{}", color::Fg(color::Red), next_key);
        }
        self.position += 1;
    }

    fn handle_backspace(&mut self) {
        // first come back before the character we’re gonna delete
        if self.position == 0 {
            return;
        }
        print!("{}", cursor::Left(1));
        self.position -= 1;
        // reprint this character in white
        print!("{}", color::Fg(color::Reset));
        print!("{}", self.text.iter().nth(self.position).unwrap());
        print!("{}", cursor::Left(1));
        if self.error.is_some() && self.error.unwrap() == self.position {
            self.error = None;
        }
    }
}
