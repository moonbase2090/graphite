// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Sidebar widget: the lunatui rendering of the Ratatui crate's `Sidebar`,
//! cell for cell.

use std::fmt::Write;

use crate::cells::{cell_rect, display_width, fill, mid_row, text, text_right, Small};
use crate::style::{dot_color, style, text_style};
use graphite_core::{SidebarLayout, SidebarRow, Tokens, SIDEBAR_ACTIONS};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// Draws a [`SidebarLayout`] with its visible rows.
pub struct Sidebar<'a> {
    layout: &'a SidebarLayout,
    rows: &'a [SidebarRow<'a>],
    tokens: &'a Tokens,
}

impl<'a> Sidebar<'a> {
    /// Creates the widget. `rows` is the full list; the layout's `first_row` picks the visible slice.
    #[must_use]
    pub fn new(layout: &'a SidebarLayout, rows: &'a [SidebarRow<'a>], tokens: &'a Tokens) -> Self {
        Self {
            layout,
            rows,
            tokens,
        }
    }
}

impl Widget for Sidebar<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let (t, l) = (self.tokens, self.layout);
        let Some(column) = cell_rect(area, l.column) else {
            return;
        };
        fill(cx, column, style(t.text, t.bar));
        if let Some(head) = cell_rect(area, l.head) {
            let rule = Rect::new(head.x, head.y + head.height - 1, head.width, 1);
            fill(cx, rule, style(t.bar_line, t.bar_line));
            text(
                cx,
                head.x + 1,
                mid_row(head),
                head.x + head.width,
                "Workspace",
                text_style(t.text_strong, t.bar),
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
            fill(cx, r, style(fg, bg));
            let y = mid_row(r);
            let right_edge = r.x + r.width;
            let mut x = r.x + 1 + 2 * u16::try_from(row.depth).unwrap_or(u16::MAX);
            if let Some(open) = row.chevron {
                text(
                    cx,
                    x,
                    y,
                    right_edge,
                    if open { "▾" } else { "▸" },
                    text_style(t.muted, bg),
                );
            }
            x += 2;
            if let Some(dot) = row.dot {
                text(cx, x, y, right_edge, "●", style(dot_color(t, dot), bg));
            }
            x += 2;
            let mut right = right_edge.saturating_sub(1);
            let mut badge = Small::new();
            if row.needs_you > 0 {
                let _ = write!(badge, "{} needs you", row.needs_you);
                text_right(cx, right, y, x, badge.as_str(), text_style(t.attention, bg));
                right = right.saturating_sub(display_width(badge.as_str()) + 1);
            } else if row.mail > 0 {
                let _ = write!(badge, "{} mail", row.mail);
                text_right(
                    cx,
                    right,
                    y,
                    x,
                    badge.as_str(),
                    text_style(t.unseen_text, bg),
                );
                right = right.saturating_sub(display_width(badge.as_str()) + 1);
            }
            text(cx, x, y, right.max(x), row.label, text_style(fg, bg));
        }
        if let Some(thumb) = l.thumb.and_then(|th| cell_rect(area, th)) {
            fill(cx, thumb, style(t.separator, t.separator));
        }
        if let Some(foot) = cell_rect(area, l.foot) {
            let rule = Rect::new(foot.x, foot.y, foot.width, 1);
            fill(cx, rule, style(t.status_line, t.status_line));
        }
        for (slot, label) in l.actions.iter().zip(SIDEBAR_ACTIONS) {
            if let Some(r) = cell_rect(area, *slot) {
                text(
                    cx,
                    r.x + 1,
                    mid_row(r),
                    r.x + r.width,
                    label,
                    text_style(t.muted, t.bar),
                );
            }
        }
    }

    fn measure(&self, avail: Size) -> Size {
        avail
    }
}
