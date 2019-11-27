use super::french::French;

pub trait Bepo {
    fn bepo_french_preset(&mut self);
}

impl Bepo for crate::Text {
    fn bepo_french_preset(&mut self) {
        self.use_french_quote();
        self.use_french_double_quote();
        self.use_french_unbreakable_space();
        self.add_ligature();
    }
}
