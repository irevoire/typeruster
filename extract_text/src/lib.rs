mod mots_surannes;

pub fn get_text() -> String {
    mots_surannes::get_text().trim().to_string()
}

pub fn use_french_quote(text: String) -> String {
    text.chars()
        .map(|c| match c {
            '\'' => '’',
            '‘' => '’',
            c => c,
        })
        .collect()
}

pub fn use_computer_quote(text: String) -> String {
    text.chars()
        .map(|c| match c {
            '’' => '\'',
            '‘' => '\'',
            c => c,
        })
        .collect()
}
