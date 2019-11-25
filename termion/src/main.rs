use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};

use engine::*;

fn main() {
    let text = extract_text::get_text();

    println!("{}", termion::screen::ToAlternateScreen);
    print!("{}", termion::cursor::Save);
    print!("{}", text);
    print!("{}", termion::cursor::Restore);

    let mut result = None;
    let mut line_end: Vec<(u16, u16)> = Vec::new();
    let mut stdin = stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    let mut engine = engine::Engine::new(&text);
    loop {
        stdout.flush().unwrap();
        let c = match stdin.next() {
            None => break,
            Some(Ok(Key::Ctrl('c'))) => {
                break;
            }
            Some(Ok(Key::Backspace)) => {
                match engine.handle_backspace() {
                    Running => continue,
                    Del(n, s) => {
                        if &s == "\n" {
                            let return_to = line_end.pop().unwrap();
                            print!("{}", cursor::Goto(return_to.0, return_to.1));
                        } else {
                            print!("{}", cursor::Left(n as u16));
                            print!("{}", color::Fg(color::Reset));
                            print!("{}", s);
                            print!("{}", cursor::Left(n as u16));
                        }
                        continue;
                    }
                };
            }
            Some(Ok(Key::Char(c))) => c,
            err => {
                println!("Unknown sequence {:?}", err);
                break;
            }
        };
        match engine.handle_keys(c) {
            Finished => {
                result = Some(engine.result());
                break;
            }
            Valid('\n') | Good('\n') | Invalid('\n') | Bad('\n') => {
                let pos = stdout.cursor_pos().unwrap();
                print!("{}", cursor::Goto(0, pos.1 + 1));
                line_end.push(pos);
            }
            Valid(c) => print!("{}{}", color::Fg(color::Green), c),
            Good(c) => print!("{}{}", color::Fg(color::Reset), c),
            Invalid(c) | Bad(c) => print!("{}{}", color::Fg(color::Red), c),
        }
    }
    stdout.suspend_raw_mode().unwrap();
    println!("{}", termion::screen::ToMainScreen);

    if result.is_some() {
        handle_result(&result.unwrap());
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
