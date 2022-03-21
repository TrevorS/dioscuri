mod client;
mod db;
mod event;
mod gemini;
mod header;
mod response;
mod tls;
mod ui;

use crate::ui::DioscuriApp;

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(DioscuriApp::default()), options);
}
