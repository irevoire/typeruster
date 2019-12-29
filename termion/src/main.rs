use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, cursor};

use engine::*;

macro_rules! color_print {
    ($writer: expr, $side:expr, $color:expr, $val:expr) => {
        write!($writer, "{}{}{}", $side($color), $val, $side(color::Reset)).unwrap();
    };
}

fn main() {
    let mut args = args::parse();
    args.apply_fix();
    let text = args.text();
    let mut result = None;

    {
        let mut screen = AlternateScreen::from(stdout());
        write!(screen, "{}", termion::cursor::Save).unwrap();
        write!(screen, "{}", text).unwrap();
        write!(screen, "{}", termion::cursor::Restore).unwrap();
        let mut screen = screen.into_raw_mode().unwrap();

        let mut stdin = stdin().keys();
        screen.flush().unwrap();

        // the first two u16 indicates the position of the cursor, so in case of error we can go back
        // to the end of the line. The boolean at the end indicates if there was an error during the
        // switch of line. This way we know if we need to delete a red square.
        let mut line_end: Vec<(u16, u16, bool)> = Vec::new();

        let mut engine = engine::Engine::new(&text);
        loop {
            screen.flush().unwrap();
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
                                write!(screen, "{}", cursor::Goto(return_to.0, return_to.1))
                                    .unwrap();
                                screen.flush().unwrap();
                                write!(screen, " {}", cursor::Left(1 + return_to.2 as u16))
                                    .unwrap();
                            } else {
                                write!(screen, "{}", cursor::Left(n as u16)).unwrap();
                                write!(screen, "{}", s).unwrap();
                                write!(screen, "{}", cursor::Left(n as u16)).unwrap();
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
                    let pos = screen.cursor_pos().unwrap();
                    write!(screen, "{}", cursor::Goto(0, pos.1 + 1)).unwrap();
                    line_end.push((pos.0, pos.1, false));
                }
                Invalid('\n') | Bad('\n') => {
                    color_print!(screen, color::Bg, color::Red, c);
                    let pos = screen.cursor_pos().unwrap();
                    write!(screen, "{}", cursor::Goto(0, pos.1 + 1)).unwrap();
                    line_end.push((pos.0, pos.1, true));
                }
                Invalid(' ') | Bad(' ') | Invalid(' ') | Bad(' ') => {
                    color_print!(screen, color::Bg, color::Red, c);
                }
                Invalid(c) | Bad(c) => color_print!(screen, color::Fg, color::Red, c),
                Valid(c) => color_print!(screen, color::Fg, color::Green, c),
                Good(c) => print!("{}", c),
            }
        }
        screen.suspend_raw_mode().unwrap();
    } // release the screen and come back to the main screen

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
