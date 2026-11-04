// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Tabs bar widget: the lunatui rendering of the same [`BarLayout`] the
//! Ratatui crate draws, cell for cell.

use crate::cells::{cell_rect, display_width, fill, mid_row, text, CELL_W};
use crate::style::{dot_color, style, text_style};
use graphite_core::{BarLayout, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// Draws a [`BarLayout`] computed by [`crate::tabs_bar_layout`].
pub struct TabsBar<'a> {
    layout: &'a BarLayout,
    tokens: &'a Tokens,
}

impl<'a> TabsBar<'a> {
    /// Creates the widget.
    #[must_use]
    pub fn new(layout: &'a BarLayout, tokens: &'a Tokens) -> Self {
        Self { layout, tokens }
    }
}

impl Widget for TabsBar<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let (t, l) = (self.tokens, self.layout);
        let Some(bar) = cell_rect(area, l.bar) else {
            return;
        };
        fill(cx, bar, style(t.text, t.bar));
        let rule = Rect::new(bar.x, bar.y + bar.height - 1, bar.width, 1);
        fill(cx, rule, style(t.bar_line, t.bar_line));
        let limit = bar.x + bar.width;

        if let Some(r) = cell_rect(area, l.dropdown) {
            fill(cx, r, style(t.text_strong, t.tab_hover));
            let (y, s) = (mid_row(r), text_style(t.text_strong, t.tab_hover));
            text(cx, r.x + 1, y, r.x + r.width, &l.space_label, s);
            let end = r.x + 1 + display_width(&l.space_label);
            text(cx, end, y, r.x + r.width, " ▾", s);
        }
        let div_x = area.x + (l.divider_x / CELL_W) as u16;
        if div_x < limit {
            text(cx, div_x, mid_row(bar), limit, "│", style(t.divider, t.bar));
        }

        for tab in &l.tabs {
            let Some(chip) = cell_rect(area, tab.chip) else {
                continue;
            };
            let bg = if tab.selected { t.tab_active } else { t.bar };
            let fg = if tab.selected {
                t.text_strong
            } else {
                t.tab_text
            };
            fill(cx, chip, style(fg, bg));
            let y = mid_row(chip);
            let end = chip.x + chip.width;
            text(cx, chip.x, y, end, " ●", style(dot_color(t, tab.dot), bg));
            let mut x = chip.x + 3;
            text(cx, x, y, end, &tab.label, text_style(fg, bg));
            x += display_width(&tab.label);
            if let Some(meta) = &tab.meta {
                text(cx, x + 1, y, end, meta, text_style(t.muted, bg));
                x += 1 + display_width(meta);
            }
            if tab.attention {
                text(
                    cx,
                    x + 1,
                    y,
                    end,
                    " needs you ",
                    text_style(t.on_attention, t.attention),
                );
            }
            if let Some(close) = cell_rect(area, tab.close) {
                text(cx, close.x, y, end, "×", text_style(t.muted, bg));
            }
        }
        if let Some(r) = cell_rect(area, l.plus) {
            text(
                cx,
                r.x,
                mid_row(r),
                r.x + r.width,
                "+",
                text_style(t.muted, t.bar),
            );
        }
        if let Some(r) = l.command.and_then(|c| cell_rect(area, c)) {
            fill(cx, r, style(t.muted, t.field));
            let y = mid_row(r);
            text(
                cx,
                r.x,
                y,
                r.x + r.width,
                " Commands",
                text_style(t.muted, t.field),
            );
            let chord_w = display_width(&l.chord) + 1;
            if r.width > chord_w + 10 {
                text(
                    cx,
                    r.x + r.width - chord_w,
                    y,
                    r.x + r.width,
                    &l.chord,
                    text_style(t.key_text, t.field),
                );
            }
        }
    }

    fn measure(&self, avail: Size) -> Size {
        Size::new(avail.width, 3.min(avail.height))
    }
}
