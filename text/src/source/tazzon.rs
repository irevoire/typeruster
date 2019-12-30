use rand::Rng;

pub fn from(id: u8) -> crate::Text {
    let url = format!(
        "http://tazzon.free.fr/dactylotest/dactylotest/new_text.php?t=26&l=fr&force={}",
        id
    );
    let text = reqwest::get(&url).unwrap().text().unwrap();
    let mut text = text.split("###").skip(1);
    let source = format!("Text number {} of tazzon (http://tazzon.free.fr/dactylotest/dactylotest).\nExtracted from: {}", id, text.next().expect("Tazzon has a bug"));
    let text = text.next().expect("Tazzon has a bug");

    crate::Text::new(text.to_string(), source.to_string())
}

pub fn random() -> crate::Text {
    let mut rng = rand::thread_rng();
    let id: u8 = rng.gen_range(0, 131);

    from(id)
}
