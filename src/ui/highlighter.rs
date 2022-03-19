use eframe::egui;
use egui::text::{LayoutJob, LayoutSection};
use egui::{Color32, FontId, Stroke, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new(syntax_set: SyntaxSet, theme_set: ThemeSet) -> Self {
        Self {
            syntax_set,
            theme_set,
        }
    }

    pub fn highlight(&self, alt_text: &str, content: &str) -> LayoutJob {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(alt_text)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        let mut hl = HighlightLines::new(syntax, theme);

        let mut job = LayoutJob {
            text: content.into(),
            ..Default::default()
        };

        for line in LinesWithEndings::from(content) {
            for (style, range) in hl.highlight(line, &self.syntax_set) {
                job.sections
                    .push(style_and_range_to_layout_section(&style, range, content));
            }
        }

        job
    }
}

fn style_and_range_to_layout_section(style: &Style, range: &str, text: &str) -> LayoutSection {
    // TODO: default size, should put this in a const or config
    let font_id = FontId::monospace(14.0);

    let color = style_to_color(style);
    let italics = style_is_italic(style);
    let underline = style_to_underline(style, &color);

    LayoutSection {
        leading_space: 0.0,
        byte_range: as_byte_range(text, range),
        format: TextFormat {
            font_id,
            color,
            italics,
            underline,
            ..Default::default()
        },
    }
}

fn as_byte_range(whole: &str, range: &str) -> std::ops::Range<usize> {
    let whole_start = whole.as_ptr() as usize;
    let range_start = range.as_ptr() as usize;

    // make sure our range isnt outside of the complete text
    assert!(whole_start <= range_start);
    assert!(range_start + range.len() <= whole_start + whole.len());

    let offset = range_start - whole_start;
    offset..(offset + range.len())
}

fn style_to_color(style: &Style) -> Color32 {
    Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b)
}

fn style_is_italic(style: &Style) -> bool {
    style.font_style.contains(FontStyle::ITALIC)
}

fn style_to_underline(style: &Style, color: &Color32) -> Stroke {
    if style.font_style.contains(FontStyle::UNDERLINE) {
        Stroke::new(1.0, *color)
    } else {
        Stroke::none()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new(
            SyntaxSet::load_defaults_newlines(),
            ThemeSet::load_defaults(),
        )
    }
}
