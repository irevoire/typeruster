use termion::{color, cursor};

pub struct Engine {
    text: Vec<char>,
    position: usize,
    // if an error was made, then error store the position were you fucked up
    error: Option<usize>,
}

pub enum State {
    Running,
    Finished(u32),
}

pub use State::*;

impl Engine {
    pub fn new(text: &str) -> Self {
        Engine {
            text: text.chars().collect(),
            position: 0,
            error: None,
        }
    }

    pub fn handle_keys(&mut self, k: char) -> State {
        let next_key = self.text.iter().nth(self.position);
        if next_key.is_none() {
            return Running;
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

        if self.position == self.text.len() {
            Finished(2)
        } else {
            Running
        }
    }

    pub fn handle_backspace(&mut self) -> State {
        // first come back before the character we’re gonna delete
        if self.position == 0 {
            return Running;
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
        Running
    }
}
