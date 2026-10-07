//! Rail and side-column geometry shared by Graphite hosts.

use crate::{Face, TextMetrics};

/// Rail chip height at 1× scale.
pub const RAIL_CHIP_H: f32 = 22.0;
/// Horizontal padding around rail chip labels at 1× scale.
pub const RAIL_CHIP_PAD_X: f32 = 9.0;
/// Rail chip text size at 1× scale.
pub const RAIL_TEXT: f32 = 12.0;
/// Rail status-dot diameter at 1× scale.
pub const RAIL_DOT: f32 = 6.0;
/// Rail chip close target width at 1× scale.
pub const RAIL_CLOSE_W: f32 = 18.0;
const RAIL_PLUS_LABEL: &str = "+ New space";

/// Returns the width of a spaces-rail chip, including its close target.
pub fn rail_chip_width<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    label: &str,
    current: bool,
) -> usize {
    let s = |design: f32| design * scale_milli as f32 / 1000.0;
    let dot = if current { s(RAIL_DOT) + s(7.0) } else { 0.0 };
    (s(RAIL_CHIP_PAD_X)
        + dot
        + metrics.width(Face::Regular, s(RAIL_TEXT), label)
        + s(6.0)
        + s(RAIL_CLOSE_W))
    .ceil() as usize
}

/// Returns the physical width of a rail chip's close target.
pub fn rail_close_width(scale_milli: u32) -> usize {
    crate::scale_px(RAIL_CLOSE_W, scale_milli)
}

/// Returns the width of the fixed `+ New space` rail button.
pub fn rail_plus_width<M: TextMetrics + ?Sized>(metrics: &M, scale_milli: u32) -> usize {
    let s = |design: f32| design * scale_milli as f32 / 1000.0;
    (2.0 * s(RAIL_CHIP_PAD_X) + metrics.width(Face::Regular, s(RAIL_TEXT), RAIL_PLUS_LABEL)).ceil()
        as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        scale_px, side_chip_px, side_cols_for_px, side_footer_px, side_gap_px, side_header_px,
        side_pad_px, side_rail_px, side_thumb_min_px, side_thumb_px, RAIL_H,
    };

    struct OneCell;

    impl TextMetrics for OneCell {
        fn width(&self, _face: Face, _px: f32, text: &str) -> f32 {
            text.chars().count() as f32
        }
    }

    #[test]
    fn side_column_geometry_matches_source_at_one_and_two_x() {
        assert_eq!(RAIL_H.px(1000), 30);
        assert_eq!(RAIL_H.px(2000), 60);
        assert_eq!(side_rail_px(1000, 18), 220);
        assert_eq!(side_rail_px(2000, 18), 440);
        assert_eq!(side_rail_px(1000, 36), 440);
        assert_eq!(side_cols_for_px(220.0, 1000, 60), 18);
        assert_eq!(side_cols_for_px(440.0, 2000, 60), 18);
        assert_eq!(side_header_px(1000), 44);
        assert_eq!(side_header_px(2000), 88);
        assert_eq!(side_footer_px(1000), 56);
        assert_eq!(side_footer_px(2000), 112);
        assert_eq!(side_chip_px(1000, true), 44);
        assert_eq!(side_chip_px(2000, true), 88);
        assert_eq!(side_chip_px(1000, false), 28);
        assert_eq!(side_chip_px(2000, false), 56);
        assert_eq!(side_gap_px(1000), 4);
        assert_eq!(side_gap_px(2000), 8);
        assert_eq!(side_pad_px(1000), 8);
        assert_eq!(side_pad_px(2000), 16);
        assert_eq!(side_thumb_px(1000), 8);
        assert_eq!(side_thumb_px(2000), 16);
        assert_eq!(side_thumb_min_px(1000), 18);
        assert_eq!(side_thumb_min_px(2000), 36);
        assert_eq!(scale_px(RAIL_CHIP_H, 1000), 22);
        assert_eq!(scale_px(RAIL_CHIP_H, 2000), 44);
        assert_eq!(scale_px(RAIL_CHIP_PAD_X, 1000), 9);
        assert_eq!(scale_px(RAIL_CHIP_PAD_X, 2000), 18);
    }

    #[test]
    fn rail_widths_use_the_injected_one_cell_measurement() {
        assert_eq!(rail_chip_width(&OneCell, 1000, "lab", false), 36);
        assert_eq!(rail_chip_width(&OneCell, 1000, "lab", true), 49);
        assert_eq!(rail_chip_width(&OneCell, 2000, "lab", false), 69);
        assert_eq!(rail_chip_width(&OneCell, 2000, "lab", true), 95);
        assert_eq!(rail_close_width(1000), 18);
        assert_eq!(rail_close_width(2000), 36);
        assert_eq!(rail_plus_width(&OneCell, 1000), 29);
        assert_eq!(rail_plus_width(&OneCell, 2000), 47);
    }
}
