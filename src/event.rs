use crossbeam::channel::{unbounded, Receiver, Sender};

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

pub type EventBroadcaster = Sender<Event>;
pub type EventRelay = Receiver<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    Back,
    Forward,
    Load(String),
    Home,
    Quit,
    Stop,
    Refresh,
}

#[derive(Debug, Clone)]
pub struct EventBus {
    channels: Vec<EventSender>,
    event_broadcaster: EventBroadcaster,
    event_relay: EventRelay,
}

impl EventBus {
    pub fn new() -> Self {
        let (event_broadcaster, event_relay) = unbounded();

        Self {
            channels: vec![],
            event_broadcaster,
            event_relay,
        }
    }

    pub fn subscribe(&mut self) -> (EventBroadcaster, EventReceiver) {
        let (tx, rx) = unbounded();

        self.channels.push(tx);

        (self.broadcaster(), rx)
    }

    pub fn broadcaster(&self) -> EventBroadcaster {
        self.event_broadcaster.clone()
    }

    pub fn relay(&self) -> anyhow::Result<()> {
        for event in self.event_relay.try_iter() {
            self.send_broadcast(event)?;
        }

        Ok(())
    }

    fn send_broadcast(&self, event: Event) -> anyhow::Result<()> {
        for channel in &self.channels {
            channel.send(event.clone())?;
        }

        Ok(())
    }
}
