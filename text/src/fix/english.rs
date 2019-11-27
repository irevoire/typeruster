pub trait English {
    fn use_english_double_quote(&mut self);
}

impl English for crate::Text {
    /// replace the double quote `"` with the real English quote: `“` and `”`.
    /// We do not change the real German or French quote: `«`, `»` and `„`.
    /// Do not change the second symbol: `″`
    fn use_english_double_quote(&mut self) {
        let mut first = true;
        self.text = self
            .text
            .chars()
            .map(|c| match (c, first) {
                ('"', true) => {
                    first = false;
                    '„'
                }
                ('"', false) => {
                    first = true;
                    '“'
                }
                (c, _) => c,
            })
            .collect();
    }
}
