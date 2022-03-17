mod viewport;

use eframe::{egui, epi};

use crate::ui::viewport::Viewport;

pub struct DioscuriApp {
    url: String,
    viewport: Viewport,
}

impl Default for DioscuriApp {
    fn default() -> Self {
        Self {
            url: "gemini://example.org".to_string(),
            viewport: Default::default(),
        }
    }
}

impl epi::App for DioscuriApp {
    fn name(&self) -> &str {
        "Dioscuri"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dioscuri");

            ui.horizontal(|ui| {
                ui.label("URL: ");
                ui.text_edit_singleline(&mut self.url);
            });

            if ui.button("Go").clicked() {
                dbg!(&self.url);
            }

            self.viewport.ui(ui);
        });

        frame.set_window_size(ctx.used_size());
    }
}
