fn main() {
    let mut engine = engine::Engine::new(&extract_text::get_text());
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
