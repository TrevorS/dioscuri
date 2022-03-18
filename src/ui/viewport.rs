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
                        if let Some(link_name) = link_name {
                            ui.hyperlink_to(link_name, url);
                        } else {
                            ui.hyperlink(url);
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
                    Line::Preformatted { alt_text: _, lines } => {
                        for line in lines {
                            if let Line::Text { content } = line {
                                ui.label(egui::RichText::new(content).monospace());
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
                sample_document().as_bytes(),
                &Url::parse("gemini://example.org").unwrap(),
            )
            .unwrap(),
        }
    }
}

fn sample_document() -> String {
    let mut d = [
        "# Sample Document",
        "Testing, testing, one two three?",
        "=> gemini://example.org Example Link",
        "> To be or not to be!",
        "## Items",
        "* Item 1",
        "* Item 2",
        "* Item 3",
        "### Check out some preformatted text:",
        "``` rust",
        "pub fn main() {",
        "   println!(\"Hello world!\");",
        "}",
        "```",
    ]
    .join("\r\n");

    d.push_str("\r\n");

    d
}
