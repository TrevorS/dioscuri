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

#[derive(Debug, Clone)]
pub struct Transceiver {
    tx: EventSender,
    rx: EventReceiver,
}

impl Transceiver {
    pub fn new(tx: EventSender, rx: EventReceiver) -> Self {
        Self { tx, rx }
    }

    pub fn send(&self, event: Event) -> anyhow::Result<()> {
        self.tx
            .send(event)
            .map_err(|e| anyhow::anyhow!("failed to send event: {}", e))
    }

    pub fn receive(&self) -> crossbeam::channel::TryIter<'_, Event> {
        self.rx.try_iter()
    }
}

pub struct EventManager {
    transceiver: Transceiver,
}

impl EventManager {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self {
            transceiver: Transceiver::new(tx, rx),
        }
    }

    pub fn connect(&self) -> Transceiver {
        self.transceiver.clone()
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}
