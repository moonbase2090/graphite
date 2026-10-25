//! Sidebar widget.

use crate::cells::{cell_rect, display_width, fill, mid_row, text, text_right};
use crate::style::{dot_color, style};
use graphite_core::{SidebarLayout, SidebarRow, Tokens, SIDEBAR_ACTIONS};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Draws a [`SidebarLayout`] with its visible rows.
pub struct Sidebar<'a> {
    layout: &'a SidebarLayout,
    rows: &'a [SidebarRow<'a>],
    tokens: &'a Tokens,
}

impl<'a> Sidebar<'a> {
    /// Creates the widget. `rows` is the full list; the layout's `first_row` picks the visible slice.
    pub fn new(layout: &'a SidebarLayout, rows: &'a [SidebarRow<'a>], tokens: &'a Tokens) -> Self {
        Self {
            layout,
            rows,
            tokens,
        }
    }
}

impl Widget for Sidebar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (t, l) = (self.tokens, self.layout);
        let Some(column) = cell_rect(area, l.column) else {
            return;
        };
        fill(buf, column, style(t.text, t.bar));
        if let Some(head) = cell_rect(area, l.head) {
            let rule = Rect::new(head.x, head.bottom() - 1, head.width, 1);
            fill(buf, rule, style(t.bar_line, t.bar_line));
            text(
                buf,
                head.x + 1,
                mid_row(head),
                head.right(),
                "Workspace",
                style(t.text_strong, t.bar),
            );
        }
        for (slot, row) in l.rows.iter().zip(self.rows.iter().skip(l.first_row)) {
            let Some(r) = cell_rect(area, *slot) else {
                continue;
            };
            let bg = if row.selected {
                t.tab_active
            } else if row.hovered {
                t.tab_hover
            } else {
                t.bar
            };
            let fg = if row.selected {
                t.text_strong
            } else {
                t.tab_text
            };
            fill(buf, r, style(fg, bg));
            let y = mid_row(r);
            let mut x = r.x + 1 + 2 * row.depth as u16;
            if let Some(open) = row.chevron {
                text(
                    buf,
                    x,
                    y,
                    r.right(),
                    if open { "▾" } else { "▸" },
                    style(t.muted, bg),
                );
            }
            x += 2;
            if let Some(dot) = row.dot {
                text(buf, x, y, r.right(), "●", style(dot_color(t, dot), bg));
            }
            x += 2;
            let mut right = r.right().saturating_sub(1);
            if row.needs_you > 0 {
                let s = format!("{} needs you", row.needs_you);
                text_right(buf, right, y, x, &s, style(t.attention, bg));
                right = right.saturating_sub(display_width(&s) + 1);
            } else if row.mail > 0 {
                let s = format!("{} mail", row.mail);
                text_right(buf, right, y, x, &s, style(t.unseen_text, bg));
                right = right.saturating_sub(display_width(&s) + 1);
            }
            text(buf, x, y, right.max(x), row.label, style(fg, bg));
        }
        if let Some(thumb) = l.thumb.and_then(|th| cell_rect(area, th)) {
            fill(buf, thumb, style(t.separator, t.separator));
        }
        if let Some(foot) = cell_rect(area, l.foot) {
            let rule = Rect::new(foot.x, foot.y, foot.width, 1);
            fill(buf, rule, style(t.status_line, t.status_line));
        }
        for (slot, label) in l.actions.iter().zip(SIDEBAR_ACTIONS) {
            if let Some(r) = cell_rect(area, *slot) {
                text(
                    buf,
                    r.x + 1,
                    mid_row(r),
                    r.right(),
                    label,
                    style(t.muted, t.bar),
                );
            }
        }
    }
}
