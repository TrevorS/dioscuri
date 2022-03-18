use eframe::egui;

#[derive(Debug, Clone, Default)]
pub struct Toolbar {
    url: String,
}

impl Toolbar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("<-").clicked() {
                dbg!("back button pressed");
            }

            if ui.button("->").clicked() {
                dbg!("forward button pressed");
            }

            if ui.button("R").clicked() {
                dbg!("refresh button pressed");
            }

            ui.text_edit_singleline(&mut self.url);
        });
    }
}
