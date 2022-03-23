use eframe::egui;
use egui::Key;

use crate::event::{Event, EventReceiver, EventSender};

#[derive(Debug, Clone)]
pub struct Toolbar {
    url: String,
    tx: EventSender,
    rx: EventReceiver,
}

impl Toolbar {
    pub fn new((tx, rx): (EventSender, EventReceiver)) -> Self {
        Self {
            url: "".to_string(),
            tx,
            rx,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Q").clicked() {
                self.tx.send(Event::Quit).unwrap();
            }

            if ui.button("<-").clicked() {
                self.tx.send(Event::Back).unwrap();
            }

            if ui.button("->").clicked() {
                self.tx.send(Event::Forward).unwrap();
            }

            if ui.button("R").clicked() {
                self.tx.send(Event::Refresh).unwrap();
            }

            if ui.button("X").clicked() {
                self.tx.send(Event::Stop).unwrap();
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                self.tx.send(Event::Load(self.url.to_string())).unwrap();
            }
        });
    }
}
