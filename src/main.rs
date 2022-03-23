mod client;
mod db;
mod event;
mod gemini;
mod header;
mod response;
mod settings;
mod tls;
mod ui;

use crate::event::EventManager;
use crate::settings::Settings;
use crate::ui::DioscuriApp;

fn main() -> anyhow::Result<()> {
    let event_manager = EventManager::new();
    let settings = Settings::new();

    let app = Box::new(DioscuriApp::new(settings, event_manager));
    eframe::run_native(app, Default::default());
}
