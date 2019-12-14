use super::computer::Computer;

pub trait Azerty {
    fn azerty_preset(&mut self);
}

impl Azerty for crate::Text {
    fn azerty_preset(&mut self) {
        self.use_computer_quote();
        self.use_computer_double_quote();
        self.use_computer_dash();
    }
}
