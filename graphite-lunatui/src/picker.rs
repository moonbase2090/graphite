// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Theme picker widget: the lunatui rendering of the Ratatui crate's
//! `Picker`, cell for cell.

use crate::cells::{cell_rect, fill, mid_row, text};
use crate::style::{style, text_style};
use graphite_core::{ThemePickerLayout, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// Draws a [`ThemePickerLayout`] with theme names.
pub struct Picker<'a> {
    layout: &'a ThemePickerLayout,
    names: &'a [&'a str],
    selected: usize,
    tokens: &'a Tokens,
}

impl<'a> Picker<'a> {
    /// Creates the widget; `selected` is an absolute index into `names`.
    #[must_use]
    pub fn new(
        layout: &'a ThemePickerLayout,
        names: &'a [&'a str],
        selected: usize,
        tokens: &'a Tokens,
    ) -> Self {
        Self {
            layout,
            names,
            selected,
            tokens,
        }
    }
}

impl Widget for Picker<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let (t, l) = (self.tokens, self.layout);
        let Some(panel) = cell_rect(area, l.panel) else {
            return;
        };
        fill(cx, panel, style(t.text, t.panel));
        let edge = style(t.field_line, t.panel);
        let limit = panel.x + panel.width;
        for x in panel.x..limit {
            text(cx, x, panel.y, limit, "─", edge);
            text(cx, x, panel.y + panel.height - 1, limit, "─", edge);
        }
        text(
            cx,
            panel.x + 2,
            panel.y,
            limit,
            " Themes ",
            text_style(t.text_strong, t.panel),
        );
        if let Some(c) = cell_rect(area, l.close) {
            text(
                cx,
                c.x,
                mid_row(c),
                c.x + c.width,
                "×",
                text_style(t.muted, t.panel),
            );
        }
        for i in 0..l.visible_rows {
            let index = l.scroll + i;
            let Some(name) = self.names.get(index) else {
                break;
            };
            let y = l.list.y + i * l.row_h;
            let Some(r) = cell_rect(
                area,
                graphite_core::Rect::new(l.list.x, y, l.list.w, l.row_h),
            ) else {
                continue;
            };
            let (fg, bg) = if index == self.selected {
                (t.text_strong, t.tab_active)
            } else {
                (t.tab_text, t.panel)
            };
            fill(cx, r, style(fg, bg));
            text(
                cx,
                r.x + 2,
                mid_row(r),
                r.x + r.width,
                name,
                text_style(fg, bg),
            );
        }
        if let Some(f) = cell_rect(
            area,
            graphite_core::Rect::new(
                l.panel.x,
                l.footer_y,
                l.panel.w,
                l.panel.y + l.panel.h - l.footer_y,
            ),
        ) {
            text(
                cx,
                f.x + 2,
                mid_row(f),
                f.x + f.width,
                "↑↓ choose · enter apply · esc close",
                text_style(t.muted, t.panel),
            );
        }
    }

    fn measure(&self, avail: Size) -> Size {
        avail
    }
}
