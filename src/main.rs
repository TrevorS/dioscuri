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

use client::GeminiClient;
use db::Db;
use tls::verification::TofuVerifier;

use crate::event::EventBus;
use crate::settings::Settings;
use crate::ui::DioscuriApp;

fn main() -> anyhow::Result<()> {
    let settings = Settings::new();

    let db = Db::new(&settings.database_path())?;
    db.prepare()?;

    let tofu_verifier = Rc::new(TofuVerifier::new(db));
    let gemini_client = GeminiClient::new(tofu_verifier)?;

    let event_bus = EventBus::new();
    let app = Box::new(DioscuriApp::new(settings, event_bus, gemini_client));
    eframe::run_native(app, Default::default());
}
