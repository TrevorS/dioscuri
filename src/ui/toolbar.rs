use eframe::egui;
use egui::Key;

#[derive(Debug, Clone)]
pub struct Toolbar {
    url: String,
}

impl Toolbar {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Q").clicked() {
                std::process::exit(0);
            }

            if ui.button("<-").clicked() {
                dbg!("back button pressed");
            }

            if ui.button("->").clicked() {
                dbg!("forward button pressed");
            }

            if ui.button("R").clicked() {
                dbg!("refresh button pressed");
            }

            if ui.button("X").clicked() {
                dbg!("stop button pressed");
            }

            let response = ui.text_edit_singleline(&mut self.url);

            if response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                dbg!(&self.url);
            }
        });
    }
}
