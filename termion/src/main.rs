use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};

use engine::*;

fn main() {
    let mut stdin = stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let text = extract_text::get_text();

    print!("{}", text); // TODO save the position of the cursor before
                        // printing the text so we can restore it right after
    print!("{}", cursor::Left(200));
    stdout.flush().unwrap();

    let mut engine = engine::Engine::new(&text);
    loop {
        stdout.flush().unwrap();
        let c = match stdin.next() {
            None => break,
            Some(Ok(Key::Ctrl('c'))) => return,
            Some(Ok(Key::Esc)) => return,
            Some(Ok(Key::Backspace)) => {
                match engine.handle_backspace() {
                    Running => continue,
                    Del(n, s) => {
                        print!("{}", cursor::Left(n as u16));
                        print!("{}", color::Fg(color::Reset));
                        print!("{}", s);
                        print!("{}", cursor::Left(n as u16));
                        continue;
                    }
                };
            }
            Some(Ok(Key::Char(c))) => c,
            err => {
                println!("Unknown sequence {:?}", err);
                return;
            }
        };
        match engine.handle_keys(c) {
            Finished => {
                stdout.suspend_raw_mode().unwrap();
                println!("{}", color::Fg(color::Reset));
                println!("You finished with 2 hits per seconds");
                break;
            }
            Valid(c) => print!("{}{}", color::Fg(color::Green), c),
            Good(c) => print!("{}{}", color::Fg(color::Reset), c),
            Invalid(c) | Bad(c) => print!("{}{}", color::Fg(color::Red), c),
        }
    }
}
