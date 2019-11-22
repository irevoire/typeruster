mod engine;
mod text;

fn main() {
    let mut engine = engine::Engine::new(&text::get_text());
    loop {
        match engine.cycle() {
            engine::Stopped => {
                println!("You stopped the race");
                break;
            }
            engine::Finished(n) => {
                println!("You finished with {} hits per seconds", n);
                break;
            }
            _ => (),
        }
    }
}
