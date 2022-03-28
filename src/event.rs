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
pub struct EventBus {
    channels: Vec<EventSender>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { channels: vec![] }
    }

    pub fn subscribe(&mut self) -> EventReceiver {
        let (tx, rx) = unbounded();

        self.channels.push(tx);

        rx
    }

    pub fn broadcast(&self, event: Event) -> anyhow::Result<()> {
        for channel in &self.channels {
            channel.send(event.clone())?;
        }

        Ok(())
    }
}
