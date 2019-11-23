pub mod result;

pub struct Engine {
    text: Vec<char>,
    position: usize,
    // if an error was made, then error store the position were you fucked up
    error: Option<usize>,
    result: crate::result::Res,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keys {
    // if a valid char was given
    Valid(char),
    // if a bad char was given move in an internal bad state
    Invalid(char),
    // if a good char was given whilst being in a bad state
    Good(char),
    // if a bad char was given whilst being in a bad state
    Bad(char),
    // no char remaining
    Finished,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delete {
    // nothing to do
    Running,
    // how much chars where deleted
    Del(usize, String),
}

pub use Delete::*;
pub use Keys::*;

impl Engine {
    pub fn new(text: &str) -> Self {
        Engine {
            text: text.chars().collect(),
            position: 0,
            error: None,
            result: crate::result::Res::new(),
        }
    }

    pub fn handle_keys(&mut self, k: char) -> Keys {
        let next_key = self.text.iter().nth(self.position);
        if next_key.is_none() {
            return Finished;
        }
        let next_key = *next_key.unwrap();
        let result = if next_key == k && self.error.is_none() {
            Valid(k)
        } else if next_key == k && self.error.is_some() {
            Good(next_key)
        } else if self.error.is_some() {
            Bad(next_key)
        } else {
            self.error = Some(self.position);
            Invalid(next_key)
        };
        self.result.keys(result);
        self.position += 1;
        result
    }

    pub fn handle_backspace(&mut self) -> Delete {
        // first come back before the character we’re gonna delete
        if self.position == 0 {
            return Running;
        }
        self.position -= 1;
        // reprint this character in white
        if self.error.is_some() && self.error.unwrap() == self.position {
            self.error = None;
        }
        self.result.delete();
        Del(1, self.text[self.position].to_string())
    }
}
