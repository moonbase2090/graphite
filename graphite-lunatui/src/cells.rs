// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Pixel-to-cell mapping shared by every widget. The mapping matches the
//! Ratatui crate cell for cell, so both backends render the same grid.

use graphite_core::{Face, Rect, TextMetrics};
use lunatui_core::{clip_display_prefix, width, Rect as CellRect, Style};
use lunatui_widgets::RenderCx;

/// Design pixels per terminal column.
pub const CELL_W: usize = 8;
/// Design pixels per terminal row.
pub const CELL_H: usize = 16;
/// Scale passed to Graphite core layouts: the terminal draws at 1×.
pub const SCALE_MILLI: u32 = 1000;

/// Measures text as terminal columns, expressed in design pixels.
#[derive(Debug, Clone, Copy, Default)]
pub struct CellMetrics;

impl TextMetrics for CellMetrics {
    fn width(&self, _face: Face, _px: f32, text: &str) -> f32 {
        (width(text) * CELL_W) as f32
    }
}

fn round_div(value: usize, by: usize) -> usize {
    (value + by / 2) / by
}

/// Maps a pixel rectangle, relative to `area`, onto cells clipped to `area`.
pub(crate) fn cell_rect(area: CellRect, r: Rect) -> Option<CellRect> {
    if r.w == 0 || r.h == 0 {
        return None;
    }
    let x0 = round_div(r.x, CELL_W);
    let y0 = round_div(r.y, CELL_H);
    let x1 = round_div(r.x + r.w, CELL_W).max(x0 + 1);
    let y1 = round_div(r.y + r.h, CELL_H).max(y0 + 1);
    let clip = |v: usize, max: u16| v.min(usize::from(max)) as u16;
    let x0 = clip(x0, area.width);
    let y0 = clip(y0, area.height);
    let x1 = clip(x1, area.width);
    let y1 = clip(y1, area.height);
    if x1 <= x0 || y1 <= y0 {
        return None;
    }
    Some(CellRect::new(area.x + x0, area.y + y0, x1 - x0, y1 - y0))
}

/// The row at the vertical middle of a cell rectangle.
pub(crate) fn mid_row(r: CellRect) -> u16 {
    r.y + r.height / 2
}

/// Clears `r` to spaces in `style`, so no glyph from an earlier frame survives.
pub(crate) fn fill(cx: &mut RenderCx<'_>, r: CellRect, style: Style) {
    cx.fill(r, style, ' ');
}

/// Draws `s` at `(x, y)` without passing column `limit`.
pub(crate) fn text(cx: &mut RenderCx<'_>, x: u16, y: u16, limit: u16, s: &str, style: Style) {
    if x >= limit {
        return;
    }
    cx.put_str(x, y, clip_display_prefix(s, limit - x), style);
}

pub(crate) fn display_width(s: &str) -> u16 {
    u16::try_from(width(s)).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_count_columns() {
        assert_eq!(CellMetrics.width(Face::Regular, 12.0, "tab"), 24.0);
        assert_eq!(CellMetrics.width(Face::Regular, 12.0, "日本"), 32.0);
    }

    #[test]
    fn rects_round_to_cells_and_clip() {
        let area = CellRect::new(2, 1, 10, 4);
        assert_eq!(
            cell_rect(area, Rect::new(8, 16, 16, 16)),
            Some(CellRect::new(3, 2, 2, 1))
        );
        assert_eq!(
            cell_rect(area, Rect::new(0, 0, 1, 1)),
            Some(CellRect::new(2, 1, 1, 1))
        );
        assert_eq!(cell_rect(area, Rect::new(800, 0, 8, 16)), None);
        assert_eq!(cell_rect(area, Rect::new(0, 0, 0, 16)), None);
    }
}
