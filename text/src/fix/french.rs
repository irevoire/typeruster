pub trait French {
    fn use_french_quote(&mut self);
}

impl French for crate::Text {
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
}
