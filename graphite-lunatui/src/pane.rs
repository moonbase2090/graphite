// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Pane chrome widget: border, title row, status. The lunatui rendering of
//! the Ratatui crate's `PaneChrome`, cell for cell.

use std::fmt::Write;

use crate::cells::{cell_rect, fill, mid_row, text, text_right, Small};
use crate::style::{style, text_style};
use graphite_core::{PaneStatus, Rect as PxRect, Tokens};
use lunatui_core::{Rect, Size};
use lunatui_widgets::{RenderCx, Widget};

/// A pane outline with a title row. The pane body is left untouched.
pub struct PaneChrome<'a> {
    slot: PxRect,
    name: &'a str,
    status: PaneStatus,
    focused: bool,
    tokens: &'a Tokens,
}

impl<'a> PaneChrome<'a> {
    /// Creates the widget for a pane `slot` given in design pixels relative to the render area.
    #[must_use]
    pub fn new(
        slot: PxRect,
        name: &'a str,
        status: PaneStatus,
        focused: bool,
        tokens: &'a Tokens,
    ) -> Self {
        Self {
            slot,
            name,
            status,
            focused,
            tokens,
        }
    }
}

impl Widget for PaneChrome<'_> {
    fn render(&self, area: Rect, cx: &mut RenderCx<'_>) {
        let t = self.tokens;
        let Some(r) = cell_rect(area, self.slot) else {
            return;
        };
        if r.width < 2 || r.height < 2 {
            return;
        }
        let edge = if self.focused {
            t.title_focus_line
        } else {
            t.hairline
        };
        let edge_style = style(edge, t.ground);
        let (x1, y1) = (r.x + r.width - 1, r.y + r.height - 1);
        let limit = r.x + r.width;
        for x in r.x + 1..x1 {
            text(cx, x, r.y, limit, "─", edge_style);
            text(cx, x, y1, limit, "─", edge_style);
        }
        for y in r.y + 1..y1 {
            text(cx, r.x, y, limit, "│", edge_style);
            text(cx, x1, y, limit, "│", edge_style);
        }
        for (x, y, s) in [
            (r.x, r.y, "╭"),
            (x1, r.y, "╮"),
            (r.x, y1, "╰"),
            (x1, y1, "╯"),
        ] {
            text(cx, x, y, limit, s, edge_style);
        }
        if r.height < 3 {
            return;
        }
        let head = Rect::new(r.x + 1, r.y + 1, r.width - 2, 1);
        let (fg, bg) = if self.focused {
            (t.text_strong, t.title_focus)
        } else {
            (t.text, t.ground)
        };
        fill(cx, head, style(fg, bg));
        let mut mail = Small::new();
        let (label, status_fg) = match self.status {
            PaneStatus::Attention => ("needs you", t.attention),
            PaneStatus::Mail(n) => {
                let _ = write!(mail, "{n} mail");
                (mail.as_str(), t.unseen_text)
            }
            PaneStatus::Unseen => ("new output", t.unseen_text),
            PaneStatus::Running => ("running", t.working),
            PaneStatus::Focused => ("focused", t.muted_focus),
            PaneStatus::Quiet => ("", t.idle),
        };
        let y = mid_row(head);
        text(
            cx,
            head.x + 1,
            y,
            head.x + head.width,
            "●",
            style(status_fg, bg),
        );
        let right = (head.x + head.width).saturating_sub(1);
        let status_w = crate::cells::display_width(label);
        let name_limit = right.saturating_sub(status_w + 1).max(head.x + 3);
        text(cx, head.x + 3, y, name_limit, self.name, text_style(fg, bg));
        if !label.is_empty() {
            text_right(cx, right, y, head.x + 3, label, text_style(status_fg, bg));
        }
    }

    fn measure(&self, avail: Size) -> Size {
        avail
    }
}
