//! Legend widget.

use crate::cells::{cell_rect, fill, mid_row, text};
use crate::style::style;
use graphite_core::{LegendPaint, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Draws the keycaps, captions, and text runs packed by [`crate::legend_layout`].
pub struct Legend<'a> {
    paint: &'a LegendPaint,
    tokens: &'a Tokens,
}

impl<'a> Legend<'a> {
    /// Creates the widget.
    pub fn new(paint: &'a LegendPaint, tokens: &'a Tokens) -> Self {
        Self { paint, tokens }
    }
}

impl Widget for Legend<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = self.tokens;
        for chip in &self.paint.chips {
            if let Some(r) = cell_rect(area, chip.rect) {
                fill(buf, r, style(t.key_text, t.key));
                text(
                    buf,
                    r.x,
                    mid_row(r),
                    r.right(),
                    &format!(" {} ", chip.label),
                    style(t.key_text, t.key),
                );
            }
        }
        for item in self.paint.texts.iter().chain(&self.paint.caption_rects) {
            if let Some(r) = cell_rect(area, item.rect) {
                text(
                    buf,
                    r.x,
                    mid_row(r),
                    area.right(),
                    &item.text,
                    style(t.muted, t.ground),
                );
            }
        }
    }
}
