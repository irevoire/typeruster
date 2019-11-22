use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};

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
            Some(Ok(Key::Ctrl('c'))) => break,
            Some(Ok(Key::Esc)) => break,
            Some(Ok(Key::Backspace)) => {
                engine.handle_backspace();
                continue;
            }
            Some(Ok(Key::Char(c))) => c,
            err => {
                println!("Unknown sequence {:?}", err);
                return;
            }
        };
        match engine.handle_keys(c) {
            engine::Finished(n) => {
                stdout.suspend_raw_mode().unwrap();
                println!("{}", color::Fg(color::Reset));
                println!("You finished with {} hits per seconds", n);
                break;
            }
            _ => (),
        }
    }
}
