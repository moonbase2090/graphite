//! Design-space dimensions and milli-scale conversion helpers.

/// A dimension expressed in Graphite design pixels at 1× scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Design(pub f32);

impl Design {
    /// Converts this design dimension to physical pixels.
    pub fn px(self, scale_milli: u32) -> usize {
        scale_px(self.0, scale_milli)
    }
}

/// Converts a design-space pixel value using the host's milli-scale rounding.
pub fn scale_px(design: f32, scale_milli: u32) -> usize {
    (design * scale_milli as f32 / 1000.0).round().max(0.0) as usize
}

/// Tabs bar height at 1× scale.
pub const TABS_BAR_H: Design = Design(44.0);
/// Pane title row height at 1× scale.
pub const PANE_HEADER_H: f32 = 28.0;
/// Spaces rail height at 1× scale.
pub const RAIL_H: Design = Design(30.0);
/// Side rail width at the default 18-column setting, at 1× scale.
pub const SIDE_RAIL_W: Design = Design(220.0);
/// Column setting that maps to [`SIDE_RAIL_W`].
pub const SIDE_RAIL_DEFAULT_COLS: f32 = 18.0;
/// Side column header height at 1× scale.
pub const SIDE_HEADER_H: f32 = 44.0;
/// Side column footer height at 1× scale.
pub const SIDE_FOOTER_H: f32 = 56.0;
/// Side-column chip height when pane names are shown.
pub const SIDE_CHIP_H: f32 = 44.0;
/// Side-column chip height when pane names are hidden.
pub const SIDE_CHIP_H_COMPACT: f32 = 28.0;
/// Gap between side-column chips.
pub const SIDE_GAP: f32 = 4.0;
/// Side-column horizontal and vertical padding.
pub const SIDE_PAD: f32 = 8.0;
/// Side-column scrollbar width.
pub const SIDE_THUMB_W: f32 = 8.0;
/// Minimum side-column scrollbar thumb height.
pub const SIDE_THUMB_MIN_H: f32 = 18.0;
/// Window edge padding at 1× scale.
pub const WINDOW_PAD: Design = Design(8.0);
/// Gap between pane slots at 1× scale.
pub const PANE_GAP: Design = Design(8.0);
/// Pane content padding at 1× scale.
pub const PANE_PAD: Design = Design(12.0);
/// Pane corner radius at 1× scale.
pub const PANE_RADIUS: f32 = 8.0;

/// Returns a scaled side-column header height.
pub fn side_header_px(scale_milli: u32) -> usize {
    scale_px(SIDE_HEADER_H, scale_milli)
}

/// Returns a scaled side-column footer height.
pub fn side_footer_px(scale_milli: u32) -> usize {
    scale_px(SIDE_FOOTER_H, scale_milli)
}

/// Returns a scaled side-column chip height.
pub fn side_chip_px(scale_milli: u32, compact: bool) -> usize {
    scale_px(
        if compact {
            SIDE_CHIP_H_COMPACT
        } else {
            SIDE_CHIP_H
        },
        scale_milli,
    )
}

/// Returns a scaled side-column gap.
pub fn side_gap_px(scale_milli: u32) -> usize {
    scale_px(SIDE_GAP, scale_milli)
}

/// Returns a scaled side-column padding.
pub fn side_pad_px(scale_milli: u32) -> usize {
    scale_px(SIDE_PAD, scale_milli)
}

/// Returns a scaled side-column scrollbar width.
pub fn side_thumb_px(scale_milli: u32) -> usize {
    scale_px(SIDE_THUMB_W, scale_milli).max(1)
}

/// Returns a scaled minimum side-column scrollbar thumb height.
pub fn side_thumb_min_px(scale_milli: u32) -> usize {
    scale_px(SIDE_THUMB_MIN_H, scale_milli).max(1)
}

/// Returns the physical side-column width for a column setting.
pub fn side_rail_px(scale_milli: u32, cols: usize) -> usize {
    let cols = (cols as f32).clamp(8.0, 60.0);
    scale_px(SIDE_RAIL_W.0 * cols / SIDE_RAIL_DEFAULT_COLS, scale_milli)
}

/// Returns the clamped column setting represented by a physical width.
pub fn side_cols_for_px(px: f64, scale_milli: u32, max_cols: usize) -> usize {
    let scale = f64::from(scale_milli.max(1)) / 1000.0;
    let per = f64::from(SIDE_RAIL_W.0) / f64::from(SIDE_RAIL_DEFAULT_COLS) * scale;
    let cols = if per <= f64::EPSILON {
        SIDE_RAIL_DEFAULT_COLS as usize
    } else {
        (px / per).round() as usize
    };
    let max_cols = max_cols.clamp(8, 60);
    cols.clamp(8, max_cols)
}

/// Sidebar width at 1× scale.
pub const SIDEBAR_W: Design = Design(256.0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_constants_match_chrome_geometry_at_one_and_two_x() {
        assert_eq!(TABS_BAR_H.px(1000), 44);
        assert_eq!(TABS_BAR_H.px(2000), 88);
        assert_eq!(SIDEBAR_W.px(1000), 256);
        assert_eq!(SIDEBAR_W.px(2000), 512);
        assert_eq!(scale_px(PANE_HEADER_H, 1000), 28);
        assert_eq!(scale_px(PANE_HEADER_H, 2000), 56);
        assert_eq!(RAIL_H.px(1000), 30);
        assert_eq!(RAIL_H.px(2000), 60);
        assert_eq!(WINDOW_PAD.px(1000), 8);
        assert_eq!(WINDOW_PAD.px(2000), 16);
        assert_eq!(PANE_GAP.px(1000), 8);
        assert_eq!(PANE_GAP.px(2000), 16);
        assert_eq!(PANE_PAD.px(1000), 12);
        assert_eq!(PANE_PAD.px(2000), 24);
        assert_eq!(scale_px(PANE_RADIUS, 1000), 8);
        assert_eq!(scale_px(PANE_RADIUS, 2000), 16);
        assert_eq!(side_header_px(1000), 44);
        assert_eq!(side_header_px(2000), 88);
        assert_eq!(side_footer_px(1000), 56);
        assert_eq!(side_footer_px(2000), 112);
        assert_eq!(side_chip_px(1000, false), 44);
        assert_eq!(side_chip_px(2000, false), 88);
        assert_eq!(side_chip_px(1000, true), 28);
        assert_eq!(side_chip_px(2000, true), 56);
        assert_eq!(side_gap_px(1000), 4);
        assert_eq!(side_gap_px(2000), 8);
        assert_eq!(side_pad_px(1000), 8);
        assert_eq!(side_pad_px(2000), 16);
        assert_eq!(side_thumb_px(1000), 8);
        assert_eq!(side_thumb_px(2000), 16);
        assert_eq!(side_thumb_min_px(1000), 18);
        assert_eq!(side_thumb_min_px(2000), 36);
        assert_eq!(scale_px(1.25, 1000), 1);
        assert_eq!(scale_px(1.25, 2000), 3);
        assert_eq!(scale_px(-1.0, 1000), 0);
    }

    #[test]
    fn small_scale_scrollbar_dimensions_keep_the_source_floor() {
        for scale_milli in [0, 1, 20, 27, 62] {
            assert_eq!(side_thumb_px(scale_milli), 1, "scale {scale_milli}");
            assert_eq!(side_thumb_min_px(scale_milli), 1, "scale {scale_milli}");
        }
    }

    #[test]
    fn side_rail_matches_the_default_width_and_scale() {
        assert_eq!(side_rail_px(1000, 18), 220);
        assert_eq!(side_rail_px(2000, 18), 440);
        assert_eq!(side_rail_px(1000, 36), 440);
        assert_eq!(side_cols_for_px(220.0, 1000, 60), 18);
        assert_eq!(side_cols_for_px(440.0, 2000, 60), 18);
    }

    #[test]
    fn side_rail_clamps_columns_to_the_host_range() {
        assert_eq!(side_rail_px(1000, 1), 98);
        assert_eq!(side_rail_px(1000, 100), 733);
        assert_eq!(side_cols_for_px(0.0, 1000, 60), 8);
        assert_eq!(side_cols_for_px(9999.0, 1000, 12), 12);
    }
}
