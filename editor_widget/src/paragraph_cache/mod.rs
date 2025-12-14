use crate::paragraph_cache::paragraph_bounds_cache::ParagraphBounds;
use buffer::Buffer;
use iced::advanced::graphics::text::Paragraph;
use iced::advanced::text::Paragraph as _;
use iced_core::alignment::Vertical;
use iced_core::text::{Alignment, LineHeight, Shaping, Wrapping};
use iced_core::{Font, Pixels, Size, Text};
use paragraph_bounds_cache::ParagraphBoundsCache;
use ropey::RopeSlice;
use std::collections::VecDeque;
use std::ops::Range;

pub mod paragraph_bounds_cache;

/// Manages creating and dropping iced Paragraphs that are visible on screen. The bounds cache
/// is used to get the lines that should be visible within the viewport and then the paragraphs
/// VecDeque has the missing paragraphs appended or dropped based on what's necessary.
///
/// When the text size changes (only width should change, height should always be f32::INFINITY),
/// the visible paragraphs need to be resized as well.
#[derive(Debug)]
pub(super) struct ParagraphCache {
    pub paragraphs: VecDeque<Paragraph>,
    pub paragraph_bounds_cache: ParagraphBoundsCache,

    /// Because the paragraphs VecDeque is a moving window we need to keep the index
    /// of the first paragraph in paragraphs here.
    pub start_line_idx: usize,
    pub last_paragraph_size: Size,
}

pub(super) struct ParagraphWithBounds<'a> {
    pub paragraph: &'a Paragraph,
    pub bounds: ParagraphBounds,
}

impl ParagraphCache {
    pub fn new_with_line_capacity(line_capacity: usize) -> Self {
        Self {
            paragraphs: VecDeque::with_capacity(64),
            paragraph_bounds_cache: ParagraphBoundsCache::with_capacity(line_capacity),
            start_line_idx: 0,
            last_paragraph_size: Default::default(),
        }
    }

    pub fn paragraphs(&mut self) -> impl Iterator<Item = ParagraphWithBounds<'_>> {
        (self.start_line_idx..(self.start_line_idx + self.paragraphs.len()))
            .into_iter()
            .map(|line_idx| ParagraphWithBounds {
                paragraph: &self.paragraphs[line_idx],
                bounds: self
                    .paragraph_bounds_cache
                    .get_bounds(line_idx)
                    .expect("Bounds to be calculated for paragraph"),
            })
    }

    pub fn scroll(&mut self, top: f32, height: f32, buffer: &Buffer, text_config: TextConfig) {
        let layout_top = (top - (height * 2.0)).max(0.0);
        let layout_bottom = top + (height * 3.0);

        let top_line = self
            .paragraph_bounds_cache
            .find_nearest_paragraph(layout_top)
            .saturating_sub(1);

        let bottom_line = self
            .paragraph_bounds_cache
            .find_nearest_paragraph(layout_bottom)
            .saturating_add(1);

        self.scroll_to_lines(top_line..bottom_line, buffer, text_config);
    }

    fn scroll_to_lines(&mut self, lines: Range<usize>, buffer: &Buffer, text_config: TextConfig) {
        {
            // Handle complete jumps (non-overlapping)
            let current_end = self.start_line_idx + self.paragraphs.len();
            if lines.start >= current_end || lines.end <= self.start_line_idx {
                self.paragraphs.clear();
                self.start_line_idx = lines.start;
            }
        }

        // Remove out of bounds paragraphs
        if self.start_line_idx < lines.start && !self.paragraphs.is_empty() {
            self.paragraphs.drain(..(lines.start - self.start_line_idx));
            self.start_line_idx = lines.start;
        }
        if (self.start_line_idx + self.paragraphs.len()) > lines.end && !self.paragraphs.is_empty()
        {
            self.paragraphs.truncate(
                lines
                    .end
                    .saturating_sub(self.start_line_idx + self.paragraphs.len()),
            );
        }

        let make_paragraph = |line_number: usize| {
            text_config.paragraph_from_line(buffer.lines_at(line_number).next().expect("In bounds"))
        };

        // Relayout existing paragraphs if the bounds changed
        for i in 0..self.paragraphs.len() {
            if text_config.bounds != self.last_paragraph_size {
                self.paragraphs[i].resize(text_config.bounds);
            }
        }

        // Fill the front (MUST use .rev() to maintain order)
        for i in (lines.start..self.start_line_idx).rev() {
            self.paragraphs.push_front(make_paragraph(i));
        }

        // Fill the back
        let current_end = self.start_line_idx + self.paragraphs.len();
        for i in current_end..lines.end {
            self.paragraphs.push_back(make_paragraph(i));
        }

        self.start_line_idx = lines.start;
        self.last_paragraph_size = text_config.bounds;
    }

    pub fn layout_everything(&mut self, buffer: &Buffer, text_config: TextConfig) {
        for (line, idx) in buffer.lines_at(0).zip(0..) {
            let paragraph = text_config.paragraph_from_line(line);
            let height = paragraph.min_height();
            self.paragraph_bounds_cache.set_height(idx, height);
        }
    }

    pub fn get_full_height_px(&mut self) -> f32 {
        let last_idx = self.paragraph_bounds_cache.lines_len() - 1;

        self.paragraph_bounds_cache.calc_bounds(last_idx);

        self.paragraph_bounds_cache
            .get_bounds(last_idx)
            .expect("Bounds to be calculated")
            .bottom()
    }
}

pub(super) struct TextConfig {
    pub bounds: Size,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: Font,
    pub align_x: Alignment,
    pub align_y: Vertical,
    pub shaping: Shaping,
    pub wrapping: Wrapping,
}

impl TextConfig {
    pub fn with_text<'a>(&'a self, text_value: &'a str) -> Text<&'a str> {
        Text {
            content: text_value,
            bounds: self.bounds,
            size: self.size,
            line_height: self.line_height,
            font: self.font,
            align_x: self.align_x,
            align_y: self.align_y,
            shaping: self.shaping,
            wrapping: self.wrapping,
        }
    }

    fn paragraph_from_line(&self, line: RopeSlice<'_>) -> Paragraph {
        Paragraph::with_text(self.with_text(line.as_str().unwrap_or(String::from(line).as_ref())))
    }
}
