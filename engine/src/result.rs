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

    /// each time a key was typed
    pub fn total_hits(&self) -> u32 {
        self.inputs.len() as u32
    }

    /// each time you typped a good key
    pub fn total_good_hits(&self) -> u32 {
        self.inputs.iter().fold(0, |acc, (ev, _)| match ev {
            Event::Key(crate::Keys::Valid(_)) | Event::Finished => acc + 1,
            _ => acc,
        })
    }

    /// each time you made an error
    pub fn total_errors(&self) -> u32 {
        self.inputs.iter().fold(0, |acc, (ev, _)| match ev {
            Event::Key(crate::Keys::Invalid(_)) => acc + 1,
            _ => acc,
        })
    }

    /// each useless key you typped because you were in an invalid state
    pub fn useless_hits(&self) -> u32 {
        self.inputs.iter().fold(0, |acc, (ev, _)| match ev {
            Event::Key(crate::Keys::Valid(_)) | Event::Finished => acc,
            _ => acc + 1,
        })
    }

    /// the percentage of mistakes, lower is better
    pub fn precision(&self) -> f32 {
        let good_hits = self.total_good_hits() as f32;
        let total = self.total_hits() as f32;
        good_hits / total * 100.0
    }

    /// total time
    pub fn total_duration(&self) -> Duration {
        self.inputs
            .iter()
            .fold(Duration::new(0, 0), |acc, (_, time)| acc + *time)
    }

    /// time lost typping errors
    pub fn time_lost_in_errors(&self) -> Duration {
        self.inputs
            .iter()
            .fold(Duration::new(0, 0), |acc, (ev, time)| match ev {
                Event::Key(crate::Keys::Valid(_)) | Event::Finished => acc,
                _ => acc + *time,
            })
    }

    /// the percentage of total time lost in errors
    pub fn time_percentage_lost_in_errors(&self) -> f32 {
        let time_lost = self.time_lost_in_errors().as_millis() as f32;
        let total = self.total_duration().as_millis() as f32;
        time_lost / total * 100.0
    }

    /// number of keypress per seconds
    pub fn hits_per_seconds(&self) -> f32 {
        self.total_hits() as f32 / self.total_duration().as_millis() as f32 * 1000.0
    }

    /// number of keypress per minutes
    pub fn hits_per_minutes(&self) -> f32 {
        self.hits_per_seconds() * 60.0
    }

    pub fn word_per_minutes(&self) -> f32 {
        self.hits_per_minutes() / 5.0
    }
}
