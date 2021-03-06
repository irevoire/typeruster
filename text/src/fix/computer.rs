pub trait Computer {
    fn use_computer_quote(&mut self);
    fn use_computer_double_quote(&mut self);
    fn use_computer_dash(&mut self);
    fn use_simple_space(&mut self);
    fn use_simple_dot(&mut self);
}

impl Computer for crate::Text {
    /// change all the strange single quote into the "normal" single quote: `'`.
    fn use_computer_quote(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '’' | '‘' => '\'',
                // prime
                '′' => '\'',
                c => c,
            })
            .collect()
    }

    /// change all the strange hyphens / dash into the "normal" dash: `-`.
    fn use_computer_dash(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '–' | '—' | '−' => '-',
                c => c,
            })
            .collect()
    }

    /// change all the strange double quote into the "normal" double quote: `"`.
    fn use_computer_double_quote(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '«' | '»' | '„' | '“' | '”' => '"',
                // second
                '″' => '"',
                c => c,
            })
            .collect()
    }

    fn use_simple_space(&mut self) {
        self.text = self
            .text
            .chars()
            .map(|c| match c {
                '\n' | '\t' => c,
                c if c.is_whitespace() => ' ',
                c => c,
            })
            .collect()
    }

    fn use_simple_dot(&mut self) {
        self.text = self.text.replace("…", "...");
    }
}
