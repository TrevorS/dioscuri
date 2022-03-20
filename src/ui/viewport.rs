use eframe::egui;
use url::Url;

use crate::gemini::{build_document, Document, Line};
use crate::ui::highlighter::SyntaxHighlighter;

pub struct Viewport {
    document: Document,
    highlighter: SyntaxHighlighter,
}

impl Viewport {
    pub fn new(document: Document, highlighter: SyntaxHighlighter) -> Self {
        Self {
            document,
            highlighter,
        }
    }

    pub fn set_document(mut self, document: Document) -> Self {
        self.document = document;

        self
    }

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
                    Line::Preformatted { alt_text, lines } => {
                        let content = extract_content_from_preformatted_line(lines);

                        match alt_text {
                            Some(alt_text) => {
                                let job = self.highlighter.highlight(alt_text, &content);
                                ui.label(job);
                            }
                            None => {
                                let text = egui::RichText::new(content).monospace();

                                ui.label(text);
                            }
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

impl Default for Viewport {
    fn default() -> Self {
        Self::new(
            build_document(
                sample_document().as_bytes(),
                &Url::parse("gemini://example.org").unwrap(),
            )
            .unwrap(),
            Default::default(),
        )
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
