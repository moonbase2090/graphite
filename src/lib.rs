//! Core types and algorithms shared by Graphite consumers.

mod color;
mod geom;
mod scale;
mod tabs_bar;
mod text;
mod tokens;

pub use color::{
    accent, bar_color_name, bar_fills, contrast_ratio, mix_rgb, relative_luminance, step_bar_color,
    BarColor,
};
pub use geom::Rect;
pub use scale::{
    scale_px, side_chip_px, side_cols_for_px, side_footer_px, side_gap_px, side_header_px,
    side_pad_px, side_rail_px, side_thumb_min_px, side_thumb_px, Design, PANE_GAP, PANE_HEADER_H,
    PANE_PAD, PANE_RADIUS, RAIL_H, SIDEBAR_W, SIDE_CHIP_H, SIDE_CHIP_H_COMPACT, SIDE_FOOTER_H,
    SIDE_GAP, SIDE_HEADER_H, SIDE_PAD, SIDE_RAIL_DEFAULT_COLS, SIDE_RAIL_W, SIDE_THUMB_MIN_H,
    SIDE_THUMB_W, TABS_BAR_H, WINDOW_PAD,
};
pub use tabs_bar::{
    bar_hit, bar_layout, shift_bar, BarHit, BarLayout, Dot, DropTarget, TabSlot, TabText,
};
pub use text::{ellipsize, Face, TextMetrics};
pub use tokens::{tokens, Rgb, ThemeVariant, Tokens, DARK, LIGHT};

#[cfg(test)]
mod tests {
    #[test]
    fn rust_version_is_pinned_to_1_85() {
        assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.85");
    }
}
