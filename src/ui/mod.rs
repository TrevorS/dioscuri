mod highlighter;
mod toolbar;
mod viewport;

use eframe::{egui, epi};
use url::Url;

use crate::client::GeminiClient;
use crate::event::{Event, EventBus, EventReceiver};
use crate::gemini::build_document;
use crate::settings::Settings;
use crate::ui::toolbar::Toolbar;
use crate::ui::viewport::Viewport;

use self::highlighter::SyntaxHighlighter;

#[derive(Debug)]
pub struct DioscuriApp<'a> {
    url: Option<Url>,
    gemini_client: GeminiClient,
    event_bus: EventBus,
    event_receiver: EventReceiver,
    settings: Settings,
    toolbar: Toolbar<'a>,
    viewport: Viewport<'a>,
    session_history: Vec<String>,
}

impl DioscuriApp<'_> {
    pub fn new(settings: Settings, mut event_bus: EventBus, gemini_client: GeminiClient) -> Self {
        let url = settings.default_url();

        let event_receiver = event_bus.subscribe();
        let toolbar = Toolbar::new(&self, event_receiver);

        let highlighter = SyntaxHighlighter::default();
        let viewport = Viewport::new(highlighter, &mut event_bus);

        event_bus
            .broadcast(Event::Load(settings.default_url().unwrap().to_string()))
            .unwrap();

        let session_history = vec![];

        Self {
            url,
            gemini_client,
            event_bus,
            event_receiver,
            settings,
            toolbar,
            viewport,
            session_history,
        }
    }

    fn process_events(&mut self) -> anyhow::Result<()> {
        for event in self.event_receiver.try_iter() {
            match event {
                Event::Back => {
                    if let Some(url) = self.session_history.pop() {
                        self.event_bus.broadcast(Event::Load(url)).unwrap();
                    };

                    dbg!("back event processed");
                }
                Event::Forward => {
                    dbg!("forward event processed");
                }
                Event::Load(url) => {
                    let url: Url = url.parse().unwrap();
                    self.url = Some(url.clone());

                    let result = self.gemini_client.get(&url).unwrap();

                    let document = build_document(result.body().unwrap(), &url).unwrap();
                    self.viewport.set_document(document);

                    self.toolbar.set_url(url.as_str());
                    self.session_history.push(url.to_string());

                    dbg!("load event processed");
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
            }
        }

        Ok(())
    }
}

impl epi::App for DioscuriApp<'_> {
    fn name(&self) -> &str {
        "Dioscuri"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.process_events().unwrap();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });

        frame.set_window_size(ctx.used_size());
    }
}
