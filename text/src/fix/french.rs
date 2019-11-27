pub trait French {
    fn use_french_quote(&mut self);
    fn use_french_double_quote(&mut self);
    fn use_french_unbreakable_space(&mut self);
}

impl French for crate::Text {
    /// replace the single quote with the french single quote `’`.
    /// Do not change the prime symbol: `′`
    fn use_french_quote(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '\'' => '’',
                '‘' => '’',
                c => c,
            })
            .collect();
    }

    /// replace the double quote `"` with the real french quote: `«` and `»`.
    /// We do not change the real english or german quote: `„`, `“` and `”`.
    /// Do not change the second symbol: `″`
    fn use_french_double_quote(&mut self) {
        let mut first = true;
        self.text = self
            .text
            .chars()
            .map(|c| match (c, first) {
                ('"', true) => {
                    first = false;
                    '«'
                }
                ('"', false) => {
                    first = true;
                    '»'
                }
                (c, _) => c,
            })
            .collect();
    }

    /// insert unbreakable space before the characters: `?`, `!`, `:` and `;`.
    /// If there is already one, does nothing
    fn use_french_unbreakable_space(&mut self) {
        let mut idx = 0;
        let mut text: Vec<char> = self.text.chars().collect();
        let mut update = false;

        while idx < text.len() {
            let c = text[idx];
            if c == '?' || c == '!' || c == ':' || c == ';' {
                if idx == 0 {
                    idx += 1;
                    continue;
                }
                update = true;
                match text[idx - 1] {
                    // unbreakable space or other punctuation
                    ' ' | '!' | '?' | ':' | ';' => {
                        idx += 1;
                        continue;
                    }
                    // normal space
                    ' ' => text[idx - 1] = ' ',
                    // every other characters
                    _ => {
                        idx += 1;
                        text.insert(idx - 1, ' ');
                    }
                }
            }
            idx += 1;
        }
        if update {
            self.text = text.iter().collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(s: &str) -> crate::Text {
        crate::Text::new(String::from(s), String::from("Author"))
    }

    #[test]
    fn test_unbreakable_space() {
        let mut text = from("Bonjour: 12");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, "Bonjour : 12");

        let mut text = from("Bonjour : 12");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, "Bonjour : 12");

        let mut text = from(": 12!?");
        text.use_french_unbreakable_space();
        assert_eq!(&text.text, ": 12 !?");
    }
}
