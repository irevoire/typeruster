use super::english::English;
use super::french::French;
use super::german::German;

pub trait Bepo {
    fn bepo_french_preset(&mut self);
    fn bepo_german_preset(&mut self);
    fn bepo_english_preset(&mut self);
}

impl Bepo for crate::Text {
    fn bepo_french_preset(&mut self) {
        self.use_french_quote();
        self.use_french_double_quote();
        self.use_french_unbreakable_space();
        self.use_french_ligatures();
    }

    fn bepo_german_preset(&mut self) {
        self.use_german_double_quote();
    }

    fn bepo_english_preset(&mut self) {
        self.use_english_double_quote();
    }
}
