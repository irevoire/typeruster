use super::computer::Computer;

pub trait Azerty {
    fn azerty_preset(&mut self);
}

impl Azerty for crate::Text {
    fn azerty_preset(&mut self) {
        self.use_computer_quote();
        self.use_only_simple_tiret();
        self.use_only_computer_double();
    }
}
