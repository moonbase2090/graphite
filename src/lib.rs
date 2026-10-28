#![doc = include_str!("../README.md")]

mod color;
#[cfg(any(feature = "motion", feature = "terminal-palette"))]
mod color_space;
#[cfg(feature = "motion")]
mod crossfade;
mod derive;
mod geom;
mod legend;
#[cfg(feature = "motion")]
mod motion;
#[cfg(feature = "terminal-palette")]
mod palette;
mod pane;
mod picker;
#[cfg(feature = "pixel-buffer")]
mod raster;
mod ring;
mod scale;
mod side_rail;
mod sidebar;
#[cfg(feature = "motion")]
mod spring;
#[cfg(feature = "states")]
mod state;
mod tabs_bar;
mod text;
#[cfg(feature = "serde")]
mod theme_file;
mod tokens;
mod translucency;

pub use color::{
    accent, bar_color_name, bar_fills, contrast_ratio, mix_rgb, relative_luminance, step_bar_color,
    BarColor, DEFAULT_ACCENT,
};
#[cfg(feature = "motion")]
pub use crossfade::{crossfade_tokens, THEME_CROSSFADE_DURATION};
pub use derive::{bar_tokens, derive_tokens, theme_tokens, uses_brief, ThemeSource};
pub use geom::Rect;
pub use legend::{legend_keys, LegendChip, LegendPaint, LegendRun, LegendText};
#[cfg(feature = "motion")]
pub use motion::{light_cycle_traced_samples, MotionCurve};
#[cfg(feature = "terminal-palette")]
pub use palette::{terminal_palette, xterm256, Ansi16, TerminalToken};
pub use pane::{activity_header_rects, pane_handle_rect, status_width, PaneStatus};
pub use picker::{
    theme_picker_hit, theme_picker_layout, theme_picker_visible_rows, ThemePickerHit,
    ThemePickerLayout,
};
#[cfg(feature = "pixel-buffer")]
pub use raster::{FramebufferError, PixelBuffer, Rgba};
pub use ring::RingSweep;
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
#[cfg(feature = "motion")]
pub use spring::{
    FrameGovernor, FrameRate, Spring, SETTLE_CELLS, SPRING_DAMPING, SPRING_MASS, SPRING_STIFFNESS,
};
#[cfg(feature = "states")]
pub use state::{state_style, ColorDepth, StateStyle, WidgetState};
pub use tabs_bar::{
    bar_hit, bar_layout, drop_target_rect, shift_bar, BarHit, BarLayout, Dot, DropTarget, TabSlot,
    TabText,
};
pub use text::{ellipsize, Face, TextMetrics};
#[cfg(feature = "serde")]
pub use theme_file::ThemeFileError;
pub use tokens::{tokens, Rgb, ThemeVariant, Tokens, DARK, LIGHT};
#[cfg(feature = "series")]
pub use tokens::{SERIES_DARK, SERIES_LIGHT};
pub use translucency::{alpha_policy, translucency_roles, AlphaPolicy, TranslucencyRole};

#[cfg(test)]
mod tests {
    #[test]
    fn rust_version_is_pinned_to_1_85() {
        assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.85");
    }
}
