use crossbeam::channel::{unbounded, Receiver, Sender};

pub enum Event {
    Back,
    Forward,
    Load,
    Quit,
    Refresh,
}

pub struct EventManager {
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self { tx, rx }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}
