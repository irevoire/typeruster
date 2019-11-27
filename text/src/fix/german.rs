pub trait German {
    fn use_german_double_quote(&mut self);
}

impl German for crate::Text {
    /// replace the double quote `"` with the real german quote: `„` and `“`.
    /// We do not change the real english or french quote: `«`, `»` and `”`.
    /// Do not change the second symbol: `″`
    fn use_german_double_quote(&mut self) {
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
