//! Pane chrome widget: border, title row, status.

use crate::cells::{cell_rect, fill, mid_row, text, text_right, Small};
use crate::style::style;
use graphite_core::{PaneStatus, Rect as PxRect, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::symbols::border::ROUNDED;
use ratatui::widgets::Widget;
use std::fmt::Write;

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
    fn render(self, area: Rect, buf: &mut Buffer) {
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
        let (x1, y1) = (r.right() - 1, r.bottom() - 1);
        for x in r.x + 1..x1 {
            text(buf, x, r.y, r.right(), "─", edge_style);
            text(buf, x, y1, r.right(), "─", edge_style);
        }
        for y in r.y + 1..y1 {
            text(buf, r.x, y, r.right(), "│", edge_style);
            text(buf, x1, y, r.right(), "│", edge_style);
        }
        for (x, y, s) in [
            (r.x, r.y, ROUNDED.top_left),
            (x1, r.y, ROUNDED.top_right),
            (r.x, y1, ROUNDED.bottom_left),
            (x1, y1, ROUNDED.bottom_right),
        ] {
            text(buf, x, y, r.right(), s, edge_style);
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
        fill(buf, head, style(fg, bg));
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
        text(buf, head.x + 1, y, head.right(), "●", style(status_fg, bg));
        let right = head.right().saturating_sub(1);
        let status_w = crate::cells::display_width(label);
        let name_limit = right.saturating_sub(status_w + 1).max(head.x + 3);
        text(buf, head.x + 3, y, name_limit, self.name, style(fg, bg));
        if !label.is_empty() {
            text_right(buf, right, y, head.x + 3, label, style(status_fg, bg));
        }
    }
}
