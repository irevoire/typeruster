use std::time::{Duration, Instant};

enum Event {
    Key(crate::Keys),
    Delete,
    Finished,
}

pub struct Res {
    inputs: Vec<(Event, Duration)>,
    last_input: Option<Instant>,
}

#[derive(Debug)]
pub struct Stats {
    /// each time a key was typed
    pub total_hits: u32,
    /// each time you made an error
    pub total_errors: u32,
    /// each useless key you typped because you were in an invalid state
    pub useless_hits: u32,
    /// hits per minutes
    pub hits_per_minutes: f32,
    /// words per minutes use a standard word size of 5
    pub words_per_minutes: f32,
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

    pub fn finished(&mut self) {
        self.event(Event::Finished);
    }

    fn event(&mut self, e: Event) {
        if let Some(t) = self.last_input {
            self.inputs.push((e, t.elapsed()));
        }
        self.last_input = Some(Instant::now());
    }

    pub fn stats(&self) -> Stats {
        let total_hits = self.inputs.len() as u32;
        let total_errors = self.inputs.iter().fold(0, |acc, (ev, _)| match ev {
            Event::Key(crate::Keys::Invalid(_)) => acc + 1,
            _ => acc,
        });
        let useless_hits = self.inputs.iter().fold(0, |acc, (ev, _)| match ev {
            Event::Key(crate::Keys::Valid(_)) | Event::Finished => acc,
            _ => acc + 1,
        });
        let total_duration = self
            .inputs
            .iter()
            .fold(Duration::new(0, 0), |acc, (_, time)| acc + *time);
        let hits_per_minutes =
            total_hits as f32 / total_duration.as_millis() as f32 * 1000.0 * 60.0;
        let words_per_minutes = hits_per_minutes / 5.0;
        Stats {
            total_hits,
            total_errors,
            useless_hits,
            hits_per_minutes,
            words_per_minutes,
        }
    }
}
