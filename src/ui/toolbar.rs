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
            if let Event::Load(url) = event {
                self.url = url;
            }
        }

        ui.horizontal(|ui| {
            if ui.button("Q").clicked() {
                self.event_broadcaster.send(Event::Quit).unwrap();
            }

            if ui.button("<-").clicked() {
                self.event_broadcaster.send(Event::Back).unwrap();
            }

            if ui.button("->").clicked() {
                self.event_broadcaster.send(Event::Forward).unwrap();
            }

            if ui.button("R").clicked() {
                self.event_broadcaster.send(Event::Refresh).unwrap();
            }

            if ui.button("H").clicked() {
                self.event_broadcaster.send(Event::Home).unwrap();
            }

            if ui.button("X").clicked() {
                self.event_broadcaster.send(Event::Stop).unwrap();
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                self.event_broadcaster
                    .send(Event::Load(self.url.to_string()))
                    .unwrap();
            }
        });
    }
}
