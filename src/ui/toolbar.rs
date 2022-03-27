use eframe::egui;
use egui::Key;

use crate::event::{Event, Transceiver};

#[derive(Debug, Clone)]
pub struct Toolbar {
    url: String,
    transceiver: Transceiver,
}

impl Toolbar {
    pub fn new(transceiver: Transceiver) -> Self {
        Self {
            url: "".to_string(),
            transceiver,
        }
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        for event in self.transceiver.receive() {
            if let Event::Load(url) = event {
                self.url = url;
            }
        }

        ui.horizontal(|ui| {
            if ui.button("Q").clicked() {
                self.transceiver.send(Event::Quit).unwrap();
            }

            if ui.button("<-").clicked() {
                self.transceiver.send(Event::Back).unwrap();
            }

            if ui.button("->").clicked() {
                self.transceiver.send(Event::Forward).unwrap();
            }

            if ui.button("R").clicked() {
                self.transceiver.send(Event::Refresh).unwrap();
            }

            if ui.button("X").clicked() {
                self.transceiver.send(Event::Stop).unwrap();
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                self.transceiver
                    .send(Event::Load(self.url.to_string()))
                    .unwrap();
            }
        });
    }
}
