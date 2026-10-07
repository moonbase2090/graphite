//! Fixed theme-picker frame geometry and absolute row hits.

use crate::Rect;

const THEME_DIALOG_W: f32 = 760.0;
const THEME_DIALOG_H: f32 = 460.0;
const THEME_HEADER_H: f32 = 56.0;
const THEME_FOOTER_H: f32 = 52.0;
const THEME_PAD: f32 = 16.0;
const THEME_ROW_H: f32 = 38.0;

/// Pure theme-picker frame geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemePickerLayout {
    /// Fixed or window-clipped panel rectangle.
    pub panel: Rect,
    /// Close-button rectangle.
    pub close: Rect,
    /// Scrollable list rectangle.
    pub list: Rect,
    /// Absolute footer y coordinate.
    pub footer_y: usize,
    /// Physical row height.
    pub row_h: usize,
    /// Number of rows visible in the list viewport.
    pub visible_rows: usize,
    /// Clamped absolute scroll offset.
    pub scroll: usize,
}

/// Theme-picker hit targets with absolute row indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePickerHit {
    /// The close control.
    Close,
    /// An absolute theme-row index.
    Row(usize),
}

/// Computes the fixed theme-picker frame and clamped scroll.
pub fn theme_picker_layout(
    scale_milli: u32,
    row_count: usize,
    scroll: usize,
    window_w: usize,
    window_h: usize,
) -> Option<ThemePickerLayout> {
    let _ = (scale_milli, row_count, scroll, window_w, window_h);
    todo!()
}

/// Returns the number of visible theme rows.
pub fn theme_picker_visible_rows(
    scale_milli: u32,
    theme_count: usize,
    window_w: usize,
    window_h: usize,
) -> usize {
    let _ = (scale_milli, theme_count, window_w, window_h);
    todo!()
}

/// Hit-tests the close control or an absolute scrolled row.
pub fn theme_picker_hit(
    scale_milli: u32,
    row_count: usize,
    scroll: usize,
    window_w: usize,
    window_h: usize,
    x: usize,
    y: usize,
) -> Option<ThemePickerHit> {
    let _ = (scale_milli, row_count, scroll, window_w, window_h, x, y);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_picker_geometry_is_fixed_and_scrolled_hits_are_absolute() {
        let short = theme_picker_layout(1000, 3, 0, 1280, 800).unwrap();
        let long = theme_picker_layout(1000, 80, 20, 1280, 800).unwrap();
        assert_eq!(short.panel, Rect::new(260, 170, 760, 460));
        assert_eq!(short.close, Rect::new(980, 186, 24, 24));
        assert_eq!(short.list, Rect::new(276, 226, 728, 352));
        assert_eq!(short.footer_y, 578);
        assert_eq!(short.row_h, 38);
        assert_eq!(short.visible_rows, 9);
        assert_eq!(short.scroll, 0);
        assert_eq!(long.panel, short.panel);
        assert_eq!(long.list, short.list);
        assert_eq!(long.visible_rows, short.visible_rows);
        assert_eq!(long.scroll, 20);
        assert_eq!(theme_picker_visible_rows(1000, 3, 1280, 800), 3);
        assert_eq!(theme_picker_visible_rows(1000, 80, 1280, 800), 9);
        assert_eq!(
            theme_picker_hit(1000, 80, 20, 1280, 800, long.list.x + 4, long.list.y + 19),
            Some(ThemePickerHit::Row(20))
        );
        assert_eq!(
            theme_picker_hit(1000, 80, 20, 1280, 800, 992, 198),
            Some(ThemePickerHit::Close)
        );
    }

    #[test]
    fn theme_picker_rejects_too_small_windows_and_empty_lists() {
        assert!(theme_picker_layout(1000, 3, 0, 319, 800).is_none());
        assert!(theme_picker_layout(1000, 3, 0, 1280, 219).is_none());
        assert_eq!(theme_picker_visible_rows(1000, 0, 1280, 800), 0);
    }
}
