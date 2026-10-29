//! Theme picker widget.

use crate::cells::{cell_rect, fill, mid_row, text};
use crate::style::{style, text_style};
use graphite_core::{ThemePickerLayout, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Draws a [`ThemePickerLayout`] with theme names.
pub struct Picker<'a> {
    layout: &'a ThemePickerLayout,
    names: &'a [&'a str],
    selected: usize,
    tokens: &'a Tokens,
}

impl<'a> Picker<'a> {
    /// Creates the widget; `selected` is an absolute index into `names`.
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (t, l) = (self.tokens, self.layout);
        let Some(panel) = cell_rect(area, l.panel) else {
            return;
        };
        fill(buf, panel, style(t.text, t.panel));
        let edge = style(t.field_line, t.panel);
        for x in panel.x..panel.right() {
            text(buf, x, panel.y, panel.right(), "─", edge);
            text(buf, x, panel.bottom() - 1, panel.right(), "─", edge);
        }
        text(
            buf,
            panel.x + 2,
            panel.y,
            panel.right(),
            " Themes ",
            text_style(t.text_strong, t.panel),
        );
        if let Some(c) = cell_rect(area, l.close) {
            text(
                buf,
                c.x,
                mid_row(c),
                c.right(),
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
            fill(buf, r, style(fg, bg));
            text(
                buf,
                r.x + 2,
                mid_row(r),
                r.right(),
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
                buf,
                f.x + 2,
                mid_row(f),
                f.right(),
                "↑↓ choose · enter apply · esc close",
                text_style(t.muted, t.panel),
            );
        }
    }
}
