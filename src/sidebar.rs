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
    let p = |design: f32| crate::scale_px(design, scale_milli);
    let head_h = crate::TABS_BAR_H.px(scale_milli);
    let action_h = p(SIDEBAR_ACTION_H);
    let foot_h = action_h
        .saturating_mul(SIDEBAR_ACTIONS.len())
        .saturating_add(p(SIDEBAR_FOOT_PAD));
    let head = Rect::new(column.x, column.y, column.w, head_h.min(column.h));
    let foot_h = foot_h.min(column.h.saturating_sub(head.h));
    let foot = Rect::new(
        column.x,
        column.y.saturating_add(column.h).saturating_sub(foot_h),
        column.w,
        foot_h,
    );
    let list_y = head.y.saturating_add(head.h);
    let list = Rect::new(column.x, list_y, column.w, foot.y.saturating_sub(list_y));
    let row_h = p(SIDEBAR_ROW_H).max(1);
    let visible = list.h / row_h;
    let max_scroll = row_count.saturating_sub(visible.max(1));
    let first_row = scroll.min(max_scroll).min(row_count);
    let mut rows = Vec::new();
    for index in 0..row_count.saturating_sub(first_row) {
        let y = list.y.saturating_add(index.saturating_mul(row_h));
        if y.saturating_add(row_h) > list.y.saturating_add(list.h) {
            break;
        }
        rows.push(Rect::new(list.x, y, list.w, row_h));
    }
    let total_h = row_count.saturating_mul(row_h);
    let thumb = if total_h > list.h && list.h > 0 {
        let thumb_h = ((list.h as f32 * list.h as f32) / total_h as f32)
            .ceil()
            .max(p(SIDEBAR_THUMB_MIN) as f32) as usize;
        let thumb_h = thumb_h.min(list.h);
        let travel = list.h.saturating_sub(thumb_h);
        let thumb_y = if max_scroll == 0 {
            list.y
        } else {
            list.y.saturating_add(
                travel
                    .saturating_mul(first_row)
                    .checked_div(max_scroll)
                    .unwrap_or(0),
            )
        };
        let thumb_w = p(SIDEBAR_THUMB_W).min(list.w);
        Some(Rect::new(
            list.x.saturating_add(list.w).saturating_sub(thumb_w),
            thumb_y,
            thumb_w,
            thumb_h,
        ))
    } else {
        None
    };
    let mut actions = [Rect::new(0, 0, 0, 0); 3];
    for (index, slot) in actions.iter_mut().enumerate() {
        *slot = Rect::new(
            foot.x,
            foot.y
                .saturating_add(p(SIDEBAR_FOOT_PAD))
                .saturating_add(index.saturating_mul(action_h)),
            foot.w,
            action_h.min(foot.h.saturating_sub(p(SIDEBAR_FOOT_PAD))),
        );
    }
    SidebarLayout {
        column,
        head,
        list,
        rows,
        thumb,
        foot,
        actions,
        first_row,
    }
}

/// Pairs absolute input rows with their visible physical slots.
pub fn sidebar_rows_in_view<'a, T>(
    rows: &'a [T],
    layout: &SidebarLayout,
) -> Vec<(usize, &'a T, Rect)> {
    rows.iter()
        .enumerate()
        .skip(layout.first_row)
        .take(layout.rows.len())
        .zip(layout.rows.iter().copied())
        .map(|((index, row), slot)| (index, row, slot))
        .collect()
}

/// Computes the sidebar header breadcrumb and fixed arrangement track.
pub fn sidebar_header_layout(scale_milli: u32, span: Rect) -> SidebarHeaderLayout {
    let p = |design: f32| crate::scale_px(design, scale_milli);
    let inset = p(ARRANGE_INSET);
    let seg_w = p(ARRANGE_SEG_W);
    let track_h = p(ARRANGE_H);
    let track_w = seg_w
        .saturating_mul(SIDEBAR_ARRANGE.len())
        .saturating_add(inset.saturating_mul(2));
    let track = Rect::new(
        span.right().saturating_sub(p(12.0)).saturating_sub(track_w),
        span.y.saturating_add(span.h.saturating_sub(track_h) / 2),
        track_w,
        track_h,
    );
    let mut buttons = [Rect::new(0, 0, 0, 0); 3];
    for (index, slot) in buttons.iter_mut().enumerate() {
        *slot = Rect::new(
            track
                .x
                .saturating_add(inset)
                .saturating_add(index.saturating_mul(seg_w)),
            track.y.saturating_add(inset),
            seg_w,
            track_h.saturating_sub(inset.saturating_mul(2)),
        );
    }
    let crumb_x = span.x.saturating_add(p(12.0));
    let crumb = Rect::new(
        crumb_x,
        span.y,
        track.x.saturating_sub(p(8.0)).saturating_sub(crumb_x),
        span.h,
    );
    SidebarHeaderLayout {
        span,
        crumb,
        track,
        buttons,
    }
}

/// Computes the sidebar collapse/expand toggle rectangle.
pub fn sidebar_toggle_rect(scale_milli: u32, column: Rect, head: Rect, dock_right: bool) -> Rect {
    let p = |design: f32| crate::scale_px(design, scale_milli);
    let size = p(18.0).max(12).min(column.w);
    let x = if dock_right {
        column.x.saturating_add(p(10.0))
    } else {
        column.right().saturating_sub(p(10.0)).saturating_sub(size)
    };
    Rect::new(
        x,
        head.y.saturating_add(head.h.saturating_sub(size) / 2),
        size,
        size,
    )
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
    if thumb.is_some_and(|rect| rect.contains(px, py)) {
        return Some(SidebarHit::Thumb);
    }
    if toggle.contains(px, py) {
        return Some(SidebarHit::Toggle);
    }
    if let Some(index) = rows.iter().position(|rect| rect.contains(px, py)) {
        return Some(SidebarHit::Row(index));
    }
    if let Some(index) = actions.iter().position(|rect| rect.contains(px, py)) {
        return Some(SidebarHit::Action(index));
    }
    arrange
        .iter()
        .position(|rect| rect.contains(px, py))
        .map(SidebarHit::Arrange)
}

/// Returns the final scroll offset for the given sidebar height and row count.
pub fn sidebar_max_scroll(scale_milli: u32, column_h: usize, row_count: usize) -> usize {
    sidebar_layout(
        scale_milli,
        Rect::new(0, 0, SIDEBAR_W.px(scale_milli), column_h),
        row_count,
        usize::MAX,
    )
    .first_row
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
        assert_eq!(one.track, Rect::new(1168, 8, 100, 28));

        let one_half = sidebar_header_layout(1500, Rect::new(256, 0, 1024, 66));
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
