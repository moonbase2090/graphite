// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Small chrome widgets: a status dot, a tab-like chip, and a count badge.

use crate::cells::{display_width, fill, text};
use crate::style::{dot_color, style, text_style};
use graphite_core::{Dot, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// One status dot in its token color.
pub struct StatusDot<'a> {
    dot: Dot,
    tokens: &'a Tokens,
}

impl<'a> StatusDot<'a> {
    /// Creates the widget.
    #[must_use]
    pub fn new(dot: Dot, tokens: &'a Tokens) -> Self {
        Self { dot, tokens }
    }
}

impl Widget for StatusDot<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let bg = self.tokens.bar;
        text(
            cx,
            area.x,
            area.y,
            area.x + 1,
            "●",
            style(dot_color(self.tokens, self.dot), bg),
        );
    }

    fn measure(&self, avail: Size) -> Size {
        Size::new(1, 1).clamp(avail)
    }
}

/// A tab-like chip: an optional status dot and a label on a filled pill.
pub struct Chip<'a> {
    label: &'a str,
    dot: Option<Dot>,
    selected: bool,
    tokens: &'a Tokens,
}

impl<'a> Chip<'a> {
    /// Creates the widget.
    #[must_use]
    pub fn new(label: &'a str, dot: Option<Dot>, selected: bool, tokens: &'a Tokens) -> Self {
        Self {
            label,
            dot,
            selected,
            tokens,
        }
    }

    fn content_width(&self) -> u16 {
        let dot = if self.dot.is_some() { 3 } else { 1 };
        dot + display_width(self.label) + 1
    }
}

impl Widget for Chip<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let t = self.tokens;
        let bg = if self.selected { t.tab_active } else { t.bar };
        let fg = if self.selected {
            t.text_strong
        } else {
            t.tab_text
        };
        fill(cx, Rect::new(area.x, area.y, area.width, 1), style(fg, bg));
        let end = area.x + area.width;
        let mut x = area.x + 1;
        if let Some(dot) = self.dot {
            text(cx, x, area.y, end, "●", style(dot_color(t, dot), bg));
            x += 2;
        }
        text(cx, x, area.y, end, self.label, text_style(fg, bg));
    }

    fn measure(&self, avail: Size) -> Size {
        Size::new(self.content_width(), 1).clamp(avail)
    }
}

/// A count pill in a status token color. A count of zero draws and measures nothing.
pub struct Badge<'a> {
    count: u32,
    dot: Dot,
    tokens: &'a Tokens,
}

impl<'a> Badge<'a> {
    /// Creates the widget.
    #[must_use]
    pub fn new(count: u32, dot: Dot, tokens: &'a Tokens) -> Self {
        Self { count, dot, tokens }
    }

    fn text(&self) -> String {
        self.count.to_string()
    }
}

impl Widget for Badge<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        if self.count == 0 || area.width == 0 || area.height == 0 {
            return;
        }
        let t = self.tokens;
        let bg = dot_color(t, self.dot);
        fill(cx, Rect::new(area.x, area.y, area.width, 1), style(bg, bg));
        text(
            cx,
            area.x + 1,
            area.y,
            area.x + area.width,
            &self.text(),
            text_style(t.on_attention, bg),
        );
    }

    fn measure(&self, avail: Size) -> Size {
        if self.count == 0 {
            return Size::new(0, 0);
        }
        Size::new(display_width(&self.text()) + 2, 1).clamp(avail)
    }
}
