//! Tabs bar widget.

use crate::cells::{cell_rect, display_width, fill, mid_row, text};
use crate::style::{dot_color, style, text_style};
use graphite_core::{BarLayout, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Draws a [`BarLayout`] computed by [`crate::tabs_bar_layout`].
pub struct TabsBar<'a> {
    layout: &'a BarLayout,
    tokens: &'a Tokens,
}

impl<'a> TabsBar<'a> {
    /// Creates the widget.
    pub fn new(layout: &'a BarLayout, tokens: &'a Tokens) -> Self {
        Self { layout, tokens }
    }
}

impl Widget for TabsBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (t, l) = (self.tokens, self.layout);
        let Some(bar) = cell_rect(area, l.bar) else {
            return;
        };
        fill(buf, bar, style(t.text, t.bar));
        let rule = Rect::new(bar.x, bar.bottom() - 1, bar.width, 1);
        fill(buf, rule, style(t.bar_line, t.bar_line));
        let limit = bar.right();

        if let Some(r) = cell_rect(area, l.dropdown) {
            fill(buf, r, style(t.text_strong, t.tab_hover));
            let (y, s) = (mid_row(r), text_style(t.text_strong, t.tab_hover));
            text(buf, r.x + 1, y, r.right(), &l.space_label, s);
            let end = r.x + 1 + display_width(&l.space_label);
            text(buf, end, y, r.right(), " ▾", s);
        }
        let div_x = area.x + (l.divider_x / crate::CELL_W) as u16;
        if div_x < limit {
            text(
                buf,
                div_x,
                mid_row(bar),
                limit,
                "│",
                style(t.divider, t.bar),
            );
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
            fill(buf, chip, style(fg, bg));
            let y = mid_row(chip);
            let end = chip.right();
            text(buf, chip.x, y, end, " ●", style(dot_color(t, tab.dot), bg));
            let mut x = chip.x + 3;
            text(buf, x, y, end, &tab.label, text_style(fg, bg));
            x += display_width(&tab.label);
            if let Some(meta) = &tab.meta {
                text(buf, x + 1, y, end, meta, text_style(t.muted, bg));
                x += 1 + display_width(meta);
            }
            if tab.attention {
                text(
                    buf,
                    x + 1,
                    y,
                    end,
                    " needs you ",
                    text_style(t.on_attention, t.attention),
                );
            }
            if let Some(close) = cell_rect(area, tab.close) {
                text(buf, close.x, y, end, "×", text_style(t.muted, bg));
            }
        }
        if let Some(r) = cell_rect(area, l.plus) {
            text(
                buf,
                r.x,
                mid_row(r),
                r.right(),
                "+",
                text_style(t.muted, t.bar),
            );
        }
        if let Some(r) = l.command.and_then(|c| cell_rect(area, c)) {
            fill(buf, r, style(t.muted, t.field));
            let y = mid_row(r);
            text(
                buf,
                r.x,
                y,
                r.right(),
                " Commands",
                text_style(t.muted, t.field),
            );
            let chord_w = display_width(&l.chord) + 1;
            if r.width > chord_w + 10 {
                text(
                    buf,
                    r.right() - chord_w,
                    y,
                    r.right(),
                    &l.chord,
                    text_style(t.key_text, t.field),
                );
            }
        }
    }
}
