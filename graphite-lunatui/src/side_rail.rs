// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Spaces rail widget: the lunatui rendering of the Ratatui crate's
//! `SideRail`, cell for cell.

use crate::cells::{fill, text, CellMetrics, CELL_W, SCALE_MILLI};
use crate::style::{style, text_style};
use graphite_core::{rail_chip_width, rail_plus_width, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// One row of space chips followed by the `+ New space` button.
pub struct SideRail<'a> {
    labels: &'a [&'a str],
    current: usize,
    tokens: &'a Tokens,
}

impl<'a> SideRail<'a> {
    /// Creates the widget; `current` indexes `labels`.
    #[must_use]
    pub fn new(labels: &'a [&'a str], current: usize, tokens: &'a Tokens) -> Self {
        Self {
            labels,
            current,
            tokens,
        }
    }
}

impl Widget for SideRail<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let t = self.tokens;
        if area.width == 0 || area.height == 0 {
            return;
        }
        fill(cx, area, style(t.muted, t.status_bar));
        if area.height > 1 {
            let rule = Rect::new(area.x, area.y, area.width, 1);
            fill(cx, rule, style(t.status_line, t.status_line));
        }
        let y = area.y + area.height / 2;
        let mut x = area.x + 1;
        let limit = area.x + area.width;
        for (i, label) in self.labels.iter().enumerate() {
            let current = i == self.current;
            let w = (rail_chip_width(&CellMetrics, SCALE_MILLI, label, current) / CELL_W) as u16;
            let (fg, bg) = if current {
                (t.text_strong, t.chip_active)
            } else {
                (t.muted, t.status_bar)
            };
            let chip_w = w.min(limit.saturating_sub(x));
            if chip_w > 0 {
                fill(cx, Rect::new(x, y, chip_w, 1), style(fg, bg));
            }
            let mut tx = x + 1;
            if current {
                text(cx, tx, y, limit, "●", style(t.working, bg));
                tx += 2;
            }
            text(cx, tx, y, limit, label, text_style(fg, bg));
            text(
                cx,
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
                cx,
                x + 1,
                y,
                limit,
                "+ New space",
                text_style(t.muted, t.status_bar),
            );
        }
    }

    fn measure(&self, avail: Size) -> Size {
        Size::new(avail.width, 2.min(avail.height))
    }
}
