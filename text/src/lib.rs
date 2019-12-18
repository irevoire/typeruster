pub mod fix;
pub mod source;

pub struct Text {
    pub text: String,
    pub source: String,
}

impl Text {
    pub fn new(text: String, source: String) -> Self {
        Text {
            text: text.trim().to_string(),
            source: source.trim().to_string(),
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
