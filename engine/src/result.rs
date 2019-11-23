use std::time::{Duration, Instant};

enum Event {
    Key(crate::Keys),
    Delete,
}

pub struct Res {
    inputs: Vec<(Event, Duration)>,
    last_input: Option<Instant>,
}

impl Res {
    pub fn new() -> Self {
        Res {
            inputs: Vec::new(),
            last_input: None,
        }
    }

    pub fn keys(&mut self, k: crate::Keys) {
        self.event(Event::Key(k));
    }

    pub fn delete(&mut self) {
        self.event(Event::Delete);
    }

    fn event(&mut self, e: Event) {
        if let Some(t) = self.last_input {
            self.inputs.push((e, t.elapsed()));
        }
        self.last_input = Some(Instant::now());
    }
}
