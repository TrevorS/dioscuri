mod highlighter;
mod toolbar;
mod viewport;

use eframe::{egui, epi};

use crate::event::EventManager;
use crate::ui::toolbar::Toolbar;
use crate::ui::viewport::Viewport;

pub struct DioscuriApp {
    url: String,
    event_manager: EventManager,
    toolbar: Toolbar,
    viewport: Viewport,
}

impl DioscuriApp {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            event_manager: Default::default(),
            toolbar: Toolbar::new(url),
            viewport: Default::default(),
        }
    }
}

impl Default for DioscuriApp {
    fn default() -> Self {
        Self::new("gemini://example.org")
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
