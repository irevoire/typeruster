use super::english::English;
use super::french::French;
use super::german::German;
use crate::fix::computer::Computer;

pub trait Bepo {
    fn bepo_french_preset(&mut self);
    fn bepo_german_preset(&mut self);
    fn bepo_english_preset(&mut self);
}

impl Bepo for crate::Text {
    fn bepo_french_preset(&mut self) {
        // first remove all the strange spacing you can get in a text
        self.use_simple_space();
        self.use_french_quote();
        self.use_french_double_quote();
        self.use_french_ligatures();
        self.use_french_three_dots();
        // write back all the unbreakable space
        self.use_french_unbreakable_space();
    }

    fn bepo_german_preset(&mut self) {
        self.use_simple_space();
        self.use_german_double_quote();
    }

    fn bepo_english_preset(&mut self) {
        self.use_simple_space();
        self.use_english_double_quote();
    }
}
