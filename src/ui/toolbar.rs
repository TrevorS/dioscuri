use eframe::egui;
use egui::Key;

use crate::event::{Event, EventReceiver};

use super::DioscuriApp;

#[derive(Debug, Clone)]
pub struct Toolbar<'a> {
    url: String,
    app: &'a DioscuriApp<'a>,
    event_receiver: EventReceiver,
}

impl<'a> Toolbar<'a> {
    pub fn new(app: &'a DioscuriApp, event_receiver: EventReceiver) -> Self {
        Self {
            url: "".to_string(),
            app,
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
                self.event_bus.broadcast(Event::Quit).unwrap();
            }

            if ui.button("<-").clicked() {
                self.event_bus.broadcast(Event::Back).unwrap();
            }

            if ui.button("->").clicked() {
                self.event_bus.broadcast(Event::Forward).unwrap();
            }

            if ui.button("R").clicked() {
                self.event_bus.broadcast(Event::Refresh).unwrap();
            }

            if ui.button("X").clicked() {
                self.event_bus.broadcast(Event::Stop).unwrap();
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                self.event_bus
                    .broadcast(Event::Load(self.url.to_string()))
                    .unwrap();
            }
        });
    }
}
