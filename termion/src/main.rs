use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};

use engine::*;
use text::fix::Bepo;

macro_rules! color_print {
    ($side:expr, $color:expr, $val:expr) => {
        print!("{}{}{}", $side($color), $val, $side(color::Reset));
    };
}

fn main() {
    let mut text = text::get_text();
    text.bepo_french_preset();

    println!("{}", termion::screen::ToAlternateScreen);
    print!("{}", termion::cursor::Save);
    print!("{}", text);
    print!("{}", termion::cursor::Restore);

    let mut result = None;
    // the first two u16 indicates the position of the cursor, so in case of error we can go back
    // to the end of the line. The boolean at the end indicates if there was an error during the
    // switch of line. This way we know if we need to delete a red square.
    let mut line_end: Vec<(u16, u16, bool)> = Vec::new();
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
                            stdout.flush().unwrap();
                            print!(" {}", cursor::Left(1 + return_to.2 as u16));
                        } else {
                            print!("{}", cursor::Left(n as u16));
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
            Valid('\n') | Good('\n') => {
                let pos = stdout.cursor_pos().unwrap();
                print!("{}", cursor::Goto(0, pos.1 + 1));
                line_end.push((pos.0, pos.1, false));
            }
            Invalid('\n') | Bad('\n') => {
                color_print!(color::Bg, color::Red, c);
                let pos = stdout.cursor_pos().unwrap();
                print!("{}", cursor::Goto(0, pos.1 + 1));
                line_end.push((pos.0, pos.1, true));
            }
            Invalid(' ') | Bad(' ') | Invalid(' ') | Bad(' ') => {
                color_print!(color::Bg, color::Red, c);
            }
            Invalid(c) | Bad(c) => color_print!(color::Fg, color::Red, c),
            Valid(c) => color_print!(color::Fg, color::Green, c),
            Good(c) => print!("{}", c),
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
    let time = result.total_duration();
    println!("Time: {}min. {}s", time.as_secs() / 60, time.as_secs() % 60);
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
        "Without errors you would’ve reached: {:.2} word per minutes.",
        result.theorical_word_per_minutes()
    );
    println!(
        "Time lost in error: {:?} ({:.2}%)",
        result.time_lost_in_errors(),
        result.time_percentage_lost_in_errors()
    );
}
