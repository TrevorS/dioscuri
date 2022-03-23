use crossbeam::channel::{unbounded, Receiver, Sender};

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    Back,
    Forward,
    Load(String),
    Quit,
    Stop,
    Refresh,
}

pub struct EventManager {
    tx: EventSender,
    rx: EventReceiver,
}

impl EventManager {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self { tx, rx }
    }

    pub fn get_tx(&self) -> Sender<Event> {
        self.tx.clone()
    }

    pub fn get_rx(&self) -> Receiver<Event> {
        self.rx.clone()
    }

    pub fn get_tx_rx(&self) -> (EventSender, EventReceiver) {
        (self.get_tx(), self.get_rx())
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}
