mod highlighter;
mod toolbar;
mod viewport;

use eframe::{egui, epi};

use crate::event::{Event, EventManager};
use crate::settings::Settings;
use crate::ui::toolbar::Toolbar;
use crate::ui::viewport::Viewport;

pub struct DioscuriApp {
    url: String,
    event_manager: EventManager,
    toolbar: Toolbar,
    settings: Settings,
    viewport: Viewport,
}

impl DioscuriApp {
    pub fn new(settings: Settings, event_manager: EventManager) -> Self {
        let url = "gemini://example.org".to_string();
        let toolbar = Toolbar::new(event_manager.get_tx_rx());

        Self {
            url,
            event_manager,
            settings,
            toolbar,
            viewport: Default::default(),
        }
    }
}

impl epi::App for DioscuriApp {
    fn name(&self) -> &str {
        "Dioscuri"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.event_manager
            .get_rx()
            .try_iter()
            .for_each(|event| match event {
                Event::Back => {
                    dbg!("back event processed");
                }
                Event::Forward => {
                    dbg!("forward event processed");
                }
                Event::Load(url) => {
                    dbg!("load event processed: {}", &url);
                }
                Event::Quit => {
                    dbg!("quit event processed");
                    std::process::exit(0);
                }
                Event::Refresh => {
                    dbg!("refresh event processed");
                }
                Event::Stop => {
                    dbg!("stop event processed");
                }
            });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });

        frame.set_window_size(ctx.used_size());
    }
}
