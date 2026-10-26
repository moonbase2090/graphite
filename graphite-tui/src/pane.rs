//! Pane chrome widget: border, title row, status.

use crate::cells::{cell_rect, fill, mid_row, text, text_right, Small};
use crate::style::style;
#[cfg(feature = "cell-design")]
use graphite_core::{BorderStyle, BLOCK_TITLE_INSET};
use graphite_core::{PaneStatus, Rect as PxRect, Tokens};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use std::fmt::Write;

#[cfg(not(feature = "cell-design"))]
const BLOCK_TITLE_INSET: u16 = 1;

#[derive(Clone, Copy)]
struct PaneBorder {
    top_left: char,
    top_right: char,
    bottom_right: char,
    bottom_left: char,
    horizontal: char,
    vertical: char,
}

#[cfg(not(feature = "cell-design"))]
impl PaneBorder {
    fn rounded() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_right: '╯',
            bottom_left: '╰',
            horizontal: '─',
            vertical: '│',
        }
    }
}

fn draw_char(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    right: u16,
    value: char,
    style: ratatui::style::Style,
) {
    let mut bytes = [0; 4];
    text(buf, x, y, right, value.encode_utf8(&mut bytes), style);
}

/// A pane outline with a title row. The pane body is left untouched.
pub struct PaneChrome<'a> {
    slot: PxRect,
    name: &'a str,
    status: PaneStatus,
    focused: bool,
    tokens: &'a Tokens,
    #[cfg(feature = "cell-design")]
    border_style: BorderStyle,
    #[cfg(feature = "cell-design")]
    ascii_borders: bool,
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
            #[cfg(feature = "cell-design")]
            border_style: BorderStyle::default(),
            #[cfg(feature = "cell-design")]
            ascii_borders: false,
        }
    }

    /// Selects the border glyph family when the `cell-design` feature is enabled.
    #[cfg(feature = "cell-design")]
    pub fn border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    /// Uses portable ASCII border glyphs when the `cell-design` feature is enabled.
    #[cfg(feature = "cell-design")]
    pub fn ascii_borders(mut self, enabled: bool) -> Self {
        self.ascii_borders = enabled;
        self
    }

    fn border(&self) -> PaneBorder {
        #[cfg(feature = "cell-design")]
        {
            let glyphs = self.border_style.glyphs(self.ascii_borders);
            PaneBorder {
                top_left: glyphs.top_left,
                top_right: glyphs.top_right,
                bottom_right: glyphs.bottom_right,
                bottom_left: glyphs.bottom_left,
                horizontal: glyphs.horizontal,
                vertical: glyphs.vertical,
            }
        }
        #[cfg(not(feature = "cell-design"))]
        PaneBorder::rounded()
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
        let border = self.border();
        let (x1, y1) = (r.right() - 1, r.bottom() - 1);
        for x in r.x + 1..x1 {
            draw_char(buf, x, r.y, r.right(), border.horizontal, edge_style);
            draw_char(buf, x, y1, r.right(), border.horizontal, edge_style);
        }
        for y in r.y + 1..y1 {
            draw_char(buf, r.x, y, r.right(), border.vertical, edge_style);
            draw_char(buf, x1, y, r.right(), border.vertical, edge_style);
        }
        for (x, y, s) in [
            (r.x, r.y, border.top_left),
            (x1, r.y, border.top_right),
            (r.x, y1, border.bottom_left),
            (x1, y1, border.bottom_right),
        ] {
            draw_char(buf, x, y, r.right(), s, edge_style);
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
        text(
            buf,
            head.x + BLOCK_TITLE_INSET,
            y,
            head.right(),
            "●",
            style(status_fg, bg),
        );
        let right = head.right().saturating_sub(1);
        let status_w = crate::cells::display_width(label);
        let title_x = head.x + BLOCK_TITLE_INSET + 2;
        let name_limit = right
            .saturating_sub(status_w + 1)
            .max(head.x + BLOCK_TITLE_INSET + 2);
        text(buf, title_x, y, name_limit, self.name, style(fg, bg));
        if !label.is_empty() {
            text_right(buf, right, y, title_x, label, style(status_fg, bg));
        }
    }
}
