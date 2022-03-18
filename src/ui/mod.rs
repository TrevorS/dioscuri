mod toolbar;
mod viewport;

use eframe::{egui, epi};

use crate::ui::toolbar::Toolbar;
use crate::ui::viewport::Viewport;

pub struct DioscuriApp {
    url: String,
    toolbar: Toolbar,
    viewport: Viewport,
}

impl Default for DioscuriApp {
    fn default() -> Self {
        Self {
            url: "gemini://example.org".to_string(),
            toolbar: Default::default(),
            viewport: Default::default(),
        }
    }
}

impl epi::App for DioscuriApp {
    fn name(&self) -> &str {
        "Dioscuri"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });

        frame.set_window_size(ctx.used_size());
    }
}
