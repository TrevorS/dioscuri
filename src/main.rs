mod client;
mod db;
mod event;
mod gemini;
mod header;
mod response;
mod settings;
mod tls;
mod ui;

use std::rc::Rc;

use log::info;

use client::GeminiClient;
use db::Db;
use event::EventBus;
use settings::Settings;
use tls::verification::TofuVerifier;
use ui::DioscuriApp;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Logging initialized");

    let settings = Settings::new();

    let db = Db::new(&settings.database_path())?;
    db.prepare()?;

    let tofu_verifier = Rc::new(TofuVerifier::new(db));
    let gemini_client = GeminiClient::new(tofu_verifier)?;

    let event_bus = EventBus::new();
    let app = Box::new(DioscuriApp::new(settings, event_bus, gemini_client));
    eframe::run_native(app, Default::default());
}
