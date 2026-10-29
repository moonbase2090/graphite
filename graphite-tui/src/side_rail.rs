//! Spaces rail widget.

use crate::cells::{fill, text, CellMetrics, CELL_W, SCALE_MILLI};
use crate::style::{style, text_style};
use graphite_core::{rail_chip_width, rail_plus_width, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// One row of space chips followed by the `+ New space` button.
pub struct SideRail<'a> {
    labels: &'a [&'a str],
    current: usize,
    tokens: &'a Tokens,
}

impl<'a> SideRail<'a> {
    /// Creates the widget; `current` indexes `labels`.
    pub fn new(labels: &'a [&'a str], current: usize, tokens: &'a Tokens) -> Self {
        Self {
            labels,
            current,
            tokens,
        }
    }
}

impl Widget for SideRail<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = self.tokens;
        if area.is_empty() {
            return;
        }
        fill(buf, area, style(t.muted, t.status_bar));
        if area.height > 1 {
            let rule = Rect::new(area.x, area.y, area.width, 1);
            fill(buf, rule, style(t.status_line, t.status_line));
        }
        let y = area.y + area.height / 2;
        let mut x = area.x + 1;
        let limit = area.right();
        for (i, label) in self.labels.iter().enumerate() {
            let current = i == self.current;
            let w = (rail_chip_width(&CellMetrics, SCALE_MILLI, label, current) / CELL_W) as u16;
            let (fg, bg) = if current {
                (t.text_strong, t.chip_active)
            } else {
                (t.muted, t.status_bar)
            };
            fill(buf, Rect::new(x, y, w, 1).intersection(area), style(fg, bg));
            let mut tx = x + 1;
            if current {
                text(buf, tx, y, limit, "●", style(t.working, bg));
                tx += 2;
            }
            text(buf, tx, y, limit, label, text_style(fg, bg));
            text(
                buf,
                x + w.saturating_sub(2),
                y,
                limit,
                "×",
                text_style(t.muted, bg),
            );
            x += w + 1;
        }
        let w = (rail_plus_width(&CellMetrics, SCALE_MILLI) / CELL_W) as u16;
        if x + w <= limit {
            text(
                buf,
                x + 1,
                y,
                limit,
                "+ New space",
                text_style(t.muted, t.status_bar),
            );
        }
    }
}
