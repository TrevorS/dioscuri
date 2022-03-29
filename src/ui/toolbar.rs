use eframe::egui;
use egui::Key;

use crate::event::{Event, EventBroadcaster, EventReceiver};

#[derive(Debug, Clone)]
pub struct Toolbar {
    url: String,
    event_broadcaster: EventBroadcaster,
    event_receiver: EventReceiver,
}

impl Toolbar {
    pub fn new(event_broadcaster: EventBroadcaster, event_receiver: EventReceiver) -> Self {
        Self {
            url: "".to_string(),
            event_broadcaster,
            event_receiver,
        }
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        for event in self.event_receiver.try_iter() {
            if let Event::Load {
                url,
                add_to_session: _,
            } = event
            {
                self.url = url;
            }
        }

        ui.horizontal(|ui| {
            if ui.button("Q").clicked() {
                self.event_broadcaster.send(Event::quit()).unwrap();
            }

            if ui.button("<-").clicked() {
                self.event_broadcaster.send(Event::back()).unwrap();
            }

            if ui.button("->").clicked() {
                self.event_broadcaster.send(Event::forward()).unwrap();
            }

            if ui.button("R").clicked() {
                self.event_broadcaster.send(Event::refresh()).unwrap();
            }

            if ui.button("H").clicked() {
                self.event_broadcaster.send(Event::home()).unwrap();
            }

            if ui.button("X").clicked() {
                self.event_broadcaster.send(Event::stop()).unwrap();
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                self.event_broadcaster.send(Event::load(&self.url)).unwrap();
            }
        });
    }
}
