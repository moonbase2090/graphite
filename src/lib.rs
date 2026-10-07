//! Core types and algorithms shared by Graphite consumers.

mod color;
mod derive;
mod geom;
mod scale;
mod side_rail;
mod sidebar;
mod tabs_bar;
mod text;
mod tokens;

pub use color::{
    accent, bar_color_name, bar_fills, contrast_ratio, mix_rgb, relative_luminance, step_bar_color,
    BarColor,
};
pub use derive::{bar_tokens, derive_tokens, theme_tokens, uses_brief, ThemeSource};
pub use geom::Rect;
pub use scale::{
    scale_px, side_chip_px, side_cols_for_px, side_footer_px, side_gap_px, side_header_px,
    side_pad_px, side_rail_px, side_thumb_min_px, side_thumb_px, Design, PANE_GAP, PANE_HEADER_H,
    PANE_PAD, PANE_RADIUS, RAIL_H, SIDEBAR_W, SIDE_CHIP_H, SIDE_CHIP_H_COMPACT, SIDE_FOOTER_H,
    SIDE_GAP, SIDE_HEADER_H, SIDE_PAD, SIDE_RAIL_DEFAULT_COLS, SIDE_RAIL_W, SIDE_THUMB_MIN_H,
    SIDE_THUMB_W, TABS_BAR_H, WINDOW_PAD,
};
pub use side_rail::{
    rail_chip_width, rail_close_width, rail_plus_width, RAIL_CHIP_H, RAIL_CHIP_PAD_X, RAIL_CLOSE_W,
    RAIL_DOT, RAIL_TEXT,
};
pub use sidebar::{
    sidebar_header_layout, sidebar_hit, sidebar_layout, sidebar_max_scroll, sidebar_rows_in_view,
    sidebar_toggle_rect, SidebarHeaderLayout, SidebarHit, SidebarLayout, SidebarRow,
    SIDEBAR_ACTIONS, SIDEBAR_ARRANGE,
};
pub use tabs_bar::{
    bar_hit, bar_layout, drop_target_rect, shift_bar, BarHit, BarLayout, Dot, DropTarget, TabSlot,
    TabText,
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
