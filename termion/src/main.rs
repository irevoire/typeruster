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

    print!("{}", termion::cursor::Save);
    print!("{}", text);
    print!("{}", termion::cursor::Restore);

    stdout.flush().unwrap();

    let mut engine = engine::Engine::new(&text);
    loop {
        stdout.flush().unwrap();
        let c = match stdin.next() {
            None => break,
            Some(Ok(Key::Ctrl('c'))) => return,
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
                handle_result(&engine.result());
                break;
            }
            Valid(c) => print!("{}{}", color::Fg(color::Green), c),
            Good(c) => print!("{}{}", color::Fg(color::Reset), c),
            Invalid(c) | Bad(c) => print!("{}{}", color::Fg(color::Red), c),
        }
    }
}

fn handle_result(result: &engine::Res) {
    println!("{}", color::Fg(color::Reset));
    println!("Time: {:.2?}", result.total_duration());
    println!(
        "You made {} mistakes ({} useless hits)",
        result.total_errors(),
        result.useless_hits()
    );
    println!("Precision: {:.2}%", result.precision());
    println!(
        "Hits per seconds: {:.2} ({:.2} hits/min)",
        result.hits_per_seconds(),
        result.hits_per_minutes()
    );
    println!("Word per minutes: {:.2}", result.word_per_minutes());
    println!(
        "Time lost in error: {:?} ({:.2}%)",
        result.time_lost_in_errors(),
        result.time_percentage_lost_in_errors()
    );
}
