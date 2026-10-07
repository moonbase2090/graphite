//! Core types and algorithms shared by Graphite consumers.

mod geom;
mod scale;
mod text;

pub use geom::Rect;
pub use scale::{
    scale_px, side_chip_px, side_cols_for_px, side_footer_px, side_gap_px, side_header_px,
    side_pad_px, side_rail_px, side_thumb_min_px, side_thumb_px, Design, PANE_GAP, PANE_HEADER_H,
    PANE_PAD, PANE_RADIUS, RAIL_H, SIDEBAR_W, SIDE_CHIP_H, SIDE_CHIP_H_COMPACT, SIDE_FOOTER_H,
    SIDE_GAP, SIDE_HEADER_H, SIDE_PAD, SIDE_RAIL_DEFAULT_COLS, SIDE_RAIL_W, SIDE_THUMB_MIN_H,
    SIDE_THUMB_W, TABS_BAR_H, WINDOW_PAD,
};
pub use text::{ellipsize, Face, TextMetrics};

#[cfg(test)]
mod tests {
    #[test]
    fn rust_version_is_pinned_to_1_85() {
        assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.85");
    }
}
