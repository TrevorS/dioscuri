mod highlighter;
mod session;
mod toolbar;
mod viewport;

use eframe::{egui, epi};
use log::{debug, info};
use url::Url;

use crate::client::GeminiClient;
use crate::event::{Event, EventBroadcaster, EventBus, EventReceiver};
use crate::gemini::build_document;
use crate::settings::Settings;
use crate::ui::session::SessionHistory;
use crate::ui::toolbar::Toolbar;
use crate::ui::viewport::Viewport;

#[derive(Debug)]
pub struct DioscuriApp {
    url: Option<Url>,
    gemini_client: GeminiClient,
    event_bus: EventBus,
    event_broadcaster: EventBroadcaster,
    event_receiver: EventReceiver,
    settings: Settings,
    toolbar: Toolbar,
    viewport: Viewport,
    session_history: SessionHistory,
}

impl DioscuriApp {
    pub fn new(settings: Settings, mut event_bus: EventBus, gemini_client: GeminiClient) -> Self {
        let url = settings.default_url();

        let (broadcaster, receiver) = event_bus.subscribe();
        let toolbar = Toolbar::new(broadcaster, receiver);

        let broadcaster = event_bus.broadcaster();
        let viewport = Viewport::new(Default::default(), broadcaster);

        let (event_broadcaster, event_receiver) = event_bus.subscribe();

        event_broadcaster
            .send(Event::load(&settings.default_url_as_string()))
            .unwrap();

        let session_history = SessionHistory::new();

        Self {
            url,
            gemini_client,
            event_bus,
            event_broadcaster,
            event_receiver,
            settings,
            toolbar,
            viewport,
            session_history,
        }
    }

    fn process_events(&mut self) -> anyhow::Result<()> {
        debug!("processing events");
        self.event_bus.relay()?;

        // TODO: extract arm logic into functions
        for event in self.event_receiver.try_iter() {
            match event {
                Event::Back => {
                    info!("processing back event");
                    info!("session history: {:#?}", &self.session_history);

                    if let Some(page) = self.session_history.go_back() {
                        info!("back - generating load event: {}", &page);

                        self.event_broadcaster
                            .send(Event::load_dont_track(page.url()))
                            .unwrap();
                    };
                }
                Event::Forward => {
                    info!("processing forward event");
                    info!("session history: {:#?}", &self.session_history);

                    if let Some(page) = self.session_history.go_forward() {
                        info!("forward - generating load event: {}", &page);

                        self.event_broadcaster
                            .send(Event::load_dont_track(page.url()))
                            .unwrap();
                    };
                }
                Event::Load {
                    url,
                    add_to_session,
                } => {
                    info!("processing load event for url: {}", url);

                    let url: Url = url.parse().unwrap();
                    self.url = Some(url.clone());

                    let result = self.gemini_client.get(&url).unwrap();
                    let document = build_document(result.body().unwrap(), &url).unwrap();

                    self.viewport.set_document(document);
                    self.toolbar.set_url(url.as_str());

                    if add_to_session {
                        self.session_history.navigate(url.as_str());
                    }
                }
                Event::Home => {
                    info!("processing home event");

                    self.event_broadcaster
                        .send(Event::load(&self.settings.default_url_as_string()))
                        .unwrap();
                }
                Event::Quit => {
                    info!("processing quit event");

                    std::process::exit(0);
                }
                Event::Refresh => {
                    info!("processing refresh event");

                    if self.url.is_some() {
                        let url = self.url.as_ref().unwrap();

                        self.event_broadcaster
                            .send(Event::load_dont_track(url.as_str()))
                            .unwrap();
                    }
                }
                Event::Stop => {
                    info!("processing stop event");
                }
            }
        }

        Ok(())
    }
}

impl epi::App for DioscuriApp {
    fn name(&self) -> &str {
        "Dioscuri"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.process_events()
            .expect("failed to process events from event_bus");

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });

        frame.set_window_size(ctx.used_size());
    }
}
