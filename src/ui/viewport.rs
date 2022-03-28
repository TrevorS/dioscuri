use eframe::egui;
use egui::RichText;

use crate::event::{Event, EventBus, EventReceiver};
use crate::gemini::{Document, Line};
use crate::ui::highlighter::SyntaxHighlighter;

#[derive(Debug)]
pub struct Viewport<'a> {
    document: Option<Document>,
    highlighter: SyntaxHighlighter,
    event_bus: &'a EventBus,
    event_receiver: EventReceiver,
}

impl<'a> Viewport<'a> {
    pub fn new(highlighter: SyntaxHighlighter, event_bus: &'a mut EventBus) -> Self {
        let event_receiver = event_bus.subscribe();

        Self {
            document: None,
            highlighter,
            event_bus,
            event_receiver,
        }
    }

    pub fn set_document(&mut self, document: Document) {
        self.document = Some(document);
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        if self.document.is_none() {
            return;
        }

        let lines = self.document.as_ref().unwrap().lines();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in lines {
                match line {
                    Line::Text { content } => {
                        ui.label(content);
                    }
                    Line::Link { url, link_name } => {
                        let response = if let Some(link_name) = link_name {
                            ui.hyperlink_to(link_name, url)
                        } else {
                            ui.hyperlink(url)
                        };

                        if response.clicked() {
                            self.event_bus
                                .broadcast(Event::Load(url.to_string()))
                                .unwrap();
                        }
                    }
                    Line::Heading { content, level: _ } => {
                        ui.label(egui::RichText::new(content).heading());
                    }
                    Line::Quote { content } => {
                        ui.label(format!("| {}", content));
                    }
                    Line::UnorderedListItem { content } => {
                        ui.label(format!("* {}", content));
                    }
                    Line::Preformatted { alt_text, lines } => {
                        let content = extract_content_from_preformatted_line(lines);

                        if let Some(alt_text) = alt_text {
                            ui.label(self.highlighter.highlight(alt_text, &content));
                        } else {
                            ui.label(RichText::new(content).monospace());
                        }
                    }
                }
            }
        });
    }
}

fn extract_content_from_preformatted_line(lines: &[Line]) -> String {
    lines
        .iter()
        .filter_map(Line::get_content)
        .collect::<Vec<&str>>()
        .join("\n")
}
