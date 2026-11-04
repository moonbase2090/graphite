// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Legend widget: the lunatui rendering of the Ratatui crate's `Legend`,
//! cell for cell.

use crate::cells::{cell_rect, fill, mid_row, text};
use crate::style::{style, text_style};
use graphite_core::{LegendPaint, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// Draws the keycaps, captions, and text runs packed by [`crate::legend_layout`].
pub struct Legend<'a> {
    paint: &'a LegendPaint,
    tokens: &'a Tokens,
}

impl<'a> Legend<'a> {
    /// Creates the widget.
    #[must_use]
    pub fn new(paint: &'a LegendPaint, tokens: &'a Tokens) -> Self {
        Self { paint, tokens }
    }
}

impl Widget for Legend<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let t = self.tokens;
        for chip in &self.paint.chips {
            if let Some(r) = cell_rect(area, chip.rect) {
                fill(cx, r, style(t.key_text, t.key));
                let (y, s) = (mid_row(r), text_style(t.key_text, t.key));
                text(cx, r.x + 1, y, r.x + r.width, &chip.label, s);
            }
        }
        for item in self.paint.texts.iter().chain(&self.paint.caption_rects) {
            if let Some(r) = cell_rect(area, item.rect) {
                text(
                    cx,
                    r.x,
                    mid_row(r),
                    area.x + area.width,
                    &item.text,
                    text_style(t.muted, t.ground),
                );
            }
        }
    }

    fn measure(&self, avail: Size) -> Size {
        avail
    }
}
