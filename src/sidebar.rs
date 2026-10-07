//! Renderer-independent sidebar layout and hit testing.

use crate::{Dot, Rect, SIDEBAR_W};

const SIDEBAR_ROW_H: f32 = 24.0;
const SIDEBAR_ACTION_H: f32 = 30.0;
const SIDEBAR_FOOT_PAD: f32 = 8.0;
const SIDEBAR_THUMB_W: f32 = 6.0;
const SIDEBAR_THUMB_MIN: f32 = 24.0;
const ARRANGE_SEG_W: f32 = 32.0;
const ARRANGE_H: f32 = 28.0;
const ARRANGE_INSET: f32 = 2.0;

/// Arrangement controls in their fixed display order.
pub const SIDEBAR_ARRANGE: [&str; 3] = ["Single", "Split", "Grid"];
/// Sidebar footer actions in their fixed display order.
pub const SIDEBAR_ACTIONS: [&str; 3] = ["+ New tab", "+ New space", "Commands"];

/// One row of host-provided sidebar content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarRow<'a> {
    /// The physical slot assigned by [`sidebar_layout`].
    pub slot: Rect,
    /// Tree depth used by the host painter for indentation.
    pub depth: usize,
    /// Optional space collapse state.
    pub chevron: Option<bool>,
    /// Optional tab status dot.
    pub dot: Option<Dot>,
    /// Display label supplied by the host.
    pub label: &'a str,
    /// Mail badge count; zero suppresses the badge.
    pub mail: u32,
    /// Needs-you badge count; zero suppresses the badge.
    pub needs_you: usize,
    /// Whether this row is selected.
    pub selected: bool,
    /// Whether this row is hovered.
    pub hovered: bool,
}

/// Fixed sidebar panel geometry and visible row slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarLayout {
    /// Full sidebar column rectangle.
    pub column: Rect,
    /// Title/header band.
    pub head: Rect,
    /// Scrollable row viewport.
    pub list: Rect,
    /// Physical row slots currently visible.
    pub rows: Vec<Rect>,
    /// Scroll thumb, when the rows overflow the viewport.
    pub thumb: Option<Rect>,
    /// Footer action region.
    pub foot: Rect,
    /// Footer action slots in [`SIDEBAR_ACTIONS`] order.
    pub actions: [Rect; 3],
    /// Absolute index of the first visible row.
    pub first_row: usize,
}

/// Sidebar header geometry over the panes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarHeaderLayout {
    /// Full header span.
    pub span: Rect,
    /// Breadcrumb region left of the arrangement controls.
    pub crumb: Rect,
    /// Fixed arrangement-control track.
    pub track: Rect,
    /// Arrangement buttons in [`SIDEBAR_ARRANGE`] order.
    pub buttons: [Rect; 3],
}

/// Index-only sidebar hit targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarHit {
    /// A visible row slot index.
    Row(usize),
    /// A footer action slot index.
    Action(usize),
    /// An arrangement button slot index.
    Arrange(usize),
    /// The sidebar collapse/expand toggle.
    Toggle,
    /// The scrollbar thumb.
    Thumb,
}

/// Computes sidebar panel, row, thumb, and action geometry.
pub fn sidebar_layout(
    scale_milli: u32,
    column: Rect,
    row_count: usize,
    scroll: usize,
) -> SidebarLayout {
    let _ = (scale_milli, column, row_count, scroll);
    todo!()
}

/// Pairs absolute input rows with their visible physical slots.
pub fn sidebar_rows_in_view<'a, T>(
    rows: &'a [T],
    layout: &SidebarLayout,
) -> Vec<(usize, &'a T, Rect)> {
    let _ = (rows, layout);
    todo!()
}

/// Computes the sidebar header breadcrumb and fixed arrangement track.
pub fn sidebar_header_layout(scale_milli: u32, span: Rect) -> SidebarHeaderLayout {
    let _ = (scale_milli, span);
    todo!()
}

/// Computes the sidebar collapse/expand toggle rectangle.
pub fn sidebar_toggle_rect(scale_milli: u32, column: Rect, head: Rect, dock_right: bool) -> Rect {
    let _ = (scale_milli, column, head, dock_right);
    todo!()
}

/// Resolves a physical point with thumb-first sidebar hit precedence.
pub fn sidebar_hit(
    rows: &[Rect],
    actions: &[Rect; 3],
    arrange: &[Rect; 3],
    thumb: Option<Rect>,
    toggle: Rect,
    px: usize,
    py: usize,
) -> Option<SidebarHit> {
    let _ = (rows, actions, arrange, thumb, toggle, px, py);
    todo!()
}

/// Returns the final scroll offset for the given sidebar height and row count.
pub fn sidebar_max_scroll(scale_milli: u32, column_h: usize, row_count: usize) -> usize {
    let _ = (scale_milli, column_h, row_count);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidebar_panel_splits_title_list_and_three_actions() {
        let layout = sidebar_layout(1000, Rect::new(0, 0, 256, 600), 4, 0);
        assert_eq!(layout.head, Rect::new(0, 0, 256, 44));
        assert_eq!(layout.foot, Rect::new(0, 502, 256, 98));
        assert_eq!(layout.list, Rect::new(0, 44, 256, 458));
        assert_eq!(
            layout.rows,
            vec![
                Rect::new(0, 44, 256, 24),
                Rect::new(0, 68, 256, 24),
                Rect::new(0, 92, 256, 24),
                Rect::new(0, 116, 256, 24),
            ]
        );
        assert_eq!(layout.thumb, None);
        assert_eq!(
            layout.actions,
            [
                Rect::new(0, 510, 256, 30),
                Rect::new(0, 540, 256, 30),
                Rect::new(0, 570, 256, 30),
            ]
        );
        assert_eq!(layout.first_row, 0);
        assert_eq!(SIDEBAR_ACTIONS, ["+ New tab", "+ New space", "Commands"]);
    }

    #[test]
    fn sidebar_scroll_clamps_and_thumbs_overflow() {
        let top = sidebar_layout(1000, Rect::new(0, 0, 256, 600), 60, 0);
        assert_eq!(top.rows.len(), 19);
        assert_eq!(top.thumb, Some(Rect::new(250, 44, 6, 146)));
        assert_eq!(top.first_row, 0);

        let bottom = sidebar_layout(1000, Rect::new(0, 0, 256, 600), 60, usize::MAX);
        assert_eq!(bottom.rows.len(), 19);
        assert_eq!(bottom.thumb, Some(Rect::new(250, 356, 6, 146)));
        assert_eq!(bottom.first_row, 41);
    }

    #[test]
    fn sidebar_rows_in_view_follow_scrolled_layout_slots() {
        let rows: Vec<usize> = (0..60).collect();
        let layout = sidebar_layout(1000, Rect::new(0, 0, 256, 600), rows.len(), usize::MAX);
        let visible = sidebar_rows_in_view(&rows, &layout);
        assert_eq!(visible.len(), 19);
        assert_eq!(
            (visible[0].0, *visible[0].1, visible[0].2),
            (41, 41, Rect::new(0, 44, 256, 24))
        );
        assert_eq!(
            (visible[18].0, *visible[18].1, visible[18].2),
            (59, 59, Rect::new(0, 476, 256, 24))
        );
    }

    #[test]
    fn sidebar_header_puts_buttons_right_of_crumb() {
        let header = sidebar_header_layout(1000, Rect::new(256, 0, 1024, 44));
        assert_eq!(header.track, Rect::new(1168, 8, 100, 28));
        assert_eq!(
            header.buttons,
            [
                Rect::new(1170, 10, 32, 24),
                Rect::new(1202, 10, 32, 24),
                Rect::new(1234, 10, 32, 24),
            ]
        );
        assert_eq!(header.crumb, Rect::new(268, 0, 892, 44));
        assert!(header.crumb.right() + 8 <= header.track.x);
    }

    #[test]
    fn arrange_control_is_a_fixed_track_of_equal_segments() {
        let one = sidebar_header_layout(1000, Rect::new(256, 0, 1024, 44));
        let one_again = sidebar_header_layout(1000, Rect::new(256, 0, 1024, 44));
        assert_eq!(one, one_again);
        assert_eq!(one.track, Rect::new(1168, 8, 100, 28));

        let one_half = sidebar_header_layout(1500, Rect::new(256, 0, 1024, 63));
        assert_eq!(one_half.track, Rect::new(1112, 12, 150, 42));
        assert_eq!(
            one_half.buttons,
            [
                Rect::new(1115, 15, 48, 36),
                Rect::new(1163, 15, 48, 36),
                Rect::new(1211, 15, 48, 36),
            ]
        );

        let two = sidebar_header_layout(2000, Rect::new(256, 0, 1024, 88));
        assert_eq!(two.track, Rect::new(1056, 16, 200, 56));
        assert_eq!(
            two.buttons,
            [
                Rect::new(1060, 20, 64, 48),
                Rect::new(1124, 20, 64, 48),
                Rect::new(1188, 20, 64, 48),
            ]
        );
    }

    #[test]
    fn sidebar_hit_prefers_thumb_then_rows_then_buttons() {
        let rows = [
            Rect::new(0, 44, 256, 24),
            Rect::new(0, 68, 256, 24),
            Rect::new(0, 92, 256, 24),
            Rect::new(0, 116, 256, 24),
        ];
        let actions = [
            Rect::new(0, 510, 256, 30),
            Rect::new(0, 540, 256, 30),
            Rect::new(0, 570, 256, 30),
        ];
        let arrange = [
            Rect::new(1170, 10, 32, 24),
            Rect::new(1202, 10, 32, 24),
            Rect::new(1234, 10, 32, 24),
        ];
        let thumb = Some(Rect::new(250, 44, 6, 146));
        let toggle = Rect::new(228, 13, 18, 18);
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 251, 46),
            Some(SidebarHit::Thumb)
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 4, 128),
            Some(SidebarHit::Row(3))
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 4, 544),
            Some(SidebarHit::Action(1))
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 1174, 14),
            Some(SidebarHit::Arrange(0))
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 900, 500),
            None
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 0, 0),
            None
        );
        assert_eq!(
            sidebar_hit(&rows, &actions, &arrange, thumb, toggle, 230, 22),
            Some(SidebarHit::Toggle)
        );
    }

    #[test]
    fn sidebar_max_scroll_is_the_last_page_offset() {
        assert_eq!(sidebar_max_scroll(1000, 600, 4), 0);
        assert_eq!(sidebar_max_scroll(1000, 600, 60), 41);
        assert_eq!(sidebar_max_scroll(2000, 600, 60), 54);
    }
}
