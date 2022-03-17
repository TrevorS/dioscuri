use eframe::egui;
use url::Url;

use crate::gemini::{build_document, Document, Line};

pub struct Viewport {
    document: Document,
}

impl Viewport {
    pub fn ui(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in self.document.lines() {
                match line {
                    Line::Text { content } => {
                        ui.label(content);
                    }
                    Line::Link { url, link_name } => {
                        ui.label(format!(
                            "Link: {} - {}",
                            link_name.as_ref().unwrap_or(&"".to_string()),
                            url,
                        ));
                    }
                    Line::Heading { content, level } => {
                        ui.label(format!("#{} {}", level, content));
                    }
                    Line::Quote { content } => {
                        ui.label(format!("> {}", content));
                    }
                    Line::UnorderedListItem { content } => {
                        ui.label(format!("* {}", content));
                    }
                    Line::Preformatted { alt_text, lines } => {
                        ui.label(format!(
                            "--- {} ---",
                            alt_text.as_ref().unwrap_or(&"".to_string())
                        ));

                        for line in lines {
                            if let Line::Text { content } = line {
                                ui.label(content);
                            } else {
                                unreachable!();
                            }
                        }
                    }
                };
            }
        });
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            document: build_document(
                "# Header\r\nText\r\n=> gemini://example.org Link\r\n> Quote!\r\n* Item!\r\n"
                    .as_bytes(),
                &Url::parse("gemini://example.org").unwrap(),
            )
            .unwrap(),
        }
    }
}
