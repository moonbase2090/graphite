//! Renderer-independent pane status and header geometry.

use crate::{Face, Rect, TextMetrics};

const HEADER_PAD_X: f32 = 12.0;
const HEADER_TEXT: f32 = 12.0;
const HEADER_DOT: f32 = 6.0;
const INNER_GAP: f32 = 8.0;
const PANE_HEADER_H: f32 = 28.0;

/// Right-hand pane-header status, ordered by urgency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneStatus {
    /// The pane needs attention.
    Attention,
    /// The pane has unread mail, with its count.
    Mail(u32),
    /// The pane has unseen output.
    Unseen,
    /// The pane is running.
    Running,
    /// The pane is focused.
    Focused,
    /// The pane is quiet.
    Quiet,
}

impl PaneStatus {
    /// Chooses status by attention, mail, unseen output, running, focus, then quiet.
    pub fn decide(attention: bool, mail: u32, unseen: bool, running: bool, focused: bool) -> Self {
        if attention {
            Self::Attention
        } else if mail > 0 {
            Self::Mail(mail)
        } else if unseen {
            Self::Unseen
        } else if running {
            Self::Running
        } else if focused {
            Self::Focused
        } else {
            Self::Quiet
        }
    }
}

/// Returns the measured width of a pane-header status label.
pub fn status_width<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    status: PaneStatus,
) -> f32 {
    let s = |design: f32| design * scale_milli as f32 / 1000.0;
    let px = s(HEADER_TEXT);
    match status {
        PaneStatus::Attention => metrics.width(Face::SemiBold, px, "needs you"),
        PaneStatus::Mail(count) => {
            s(14.0) + s(6.0) + metrics.width(Face::Regular, px, &count.to_string())
        }
        PaneStatus::Unseen => metrics.width(Face::Regular, px, "new output"),
        PaneStatus::Running => metrics.width(Face::Regular, px, "running"),
        PaneStatus::Focused => metrics.width(Face::Regular, px, "focused"),
        PaneStatus::Quiet => 0.0,
    }
}

/// Returns the dot-and-name handle zone in a pane header.
pub fn pane_handle_rect<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    slot: Rect,
    name: &str,
    status: PaneStatus,
    focus_row: bool,
) -> Option<Rect> {
    let s = |design: f32| design * scale_milli as f32 / 1000.0;
    let p = |design: f32| crate::scale_px(design, scale_milli);
    let head_h = p(PANE_HEADER_H).min(slot.h);
    if slot.w < p(40.0) || head_h == 0 {
        return None;
    }
    let x0 = slot.x as f32 + s(HEADER_PAD_X);
    let px = s(HEADER_TEXT);
    let status_w = status_width(metrics, scale_milli, status);
    let text_end = slot
        .right()
        .saturating_sub(p(HEADER_PAD_X) + status_w.ceil() as usize + p(INNER_GAP))
        as f32;
    let text_x = x0 + s(HEADER_DOT) + s(INNER_GAP);
    let face = if focus_row {
        Face::SemiBold
    } else {
        Face::Regular
    };
    let shown = crate::ellipsize(metrics, face, px, name, (text_end - text_x).max(0.0));
    let end = (text_x + metrics.width(face, px, &shown)).ceil() as usize;
    let end = end.min(text_end.ceil() as usize).max(x0.ceil() as usize);
    Some(Rect::new(
        x0.ceil() as usize,
        slot.y,
        end.saturating_sub(x0.ceil() as usize),
        head_h,
    ))
}

/// Returns damage rectangles for the pane-header activity regions.
pub fn activity_header_rects(scale_milli: u32, slot: Rect) -> Vec<Rect> {
    let p = |design: f32| crate::scale_px(design, scale_milli);
    let head_h = p(PANE_HEADER_H).min(slot.h);
    if slot.w < p(40.0) || head_h == 0 {
        return Vec::new();
    }
    let pad = p(HEADER_PAD_X);
    let dot = p(HEADER_DOT).max(1);
    let cx = slot.x.saturating_add(pad).saturating_add(dot / 2);
    let dot_left = cx.saturating_sub(dot / 2 + 2).max(slot.x);
    let dot_right = cx.saturating_add(dot / 2 + 3).min(slot.right());
    let dot_rect = Rect::new(dot_left, slot.y, dot_right.saturating_sub(dot_left), head_h);
    let status_w = p(168.0).min(slot.w);
    let status = Rect::new(
        slot.right().saturating_sub(status_w),
        slot.y,
        status_w,
        head_h,
    );
    vec![dot_rect, status]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Dot;

    struct OneCell;

    impl TextMetrics for OneCell {
        fn width(&self, _face: Face, px: f32, text: &str) -> f32 {
            text.chars().count() as f32 * px
        }
    }

    #[test]
    fn pane_status_decide_uses_the_priority_table() {
        assert_eq!(
            PaneStatus::decide(true, 3, true, true, true),
            PaneStatus::Attention
        );
        assert_eq!(
            PaneStatus::decide(false, 3, true, true, true),
            PaneStatus::Mail(3)
        );
        assert_eq!(
            PaneStatus::decide(false, 0, true, true, true),
            PaneStatus::Unseen
        );
        assert_eq!(
            PaneStatus::decide(false, 0, false, true, true),
            PaneStatus::Running
        );
        assert_eq!(
            PaneStatus::decide(false, 0, false, false, true),
            PaneStatus::Focused
        );
        assert_eq!(
            PaneStatus::decide(false, 0, false, false, false),
            PaneStatus::Quiet
        );
    }

    #[test]
    fn dot_for_tab_uses_the_priority_table() {
        assert_eq!(Dot::for_tab(true, true, true), Dot::Attention);
        assert_eq!(Dot::for_tab(false, true, true), Dot::Working);
        assert_eq!(Dot::for_tab(false, false, true), Dot::Unseen);
        assert_eq!(Dot::for_tab(false, false, false), Dot::Idle);
    }

    #[test]
    fn pane_handle_reserves_mail_status_space_before_clipping_name() {
        let metrics = OneCell;
        let slot = Rect::new(8, 40, 400, 200);
        let handle = pane_handle_rect(
            &metrics,
            1000,
            slot,
            "abcdefghijklmnopqrstuvwxyz0123456789",
            PaneStatus::Mail(12),
            false,
        )
        .unwrap();
        // Mail(12): 14px envelope + 6px gap + two count scalars at 12px,
        // plus the 12px right pad and 8px inner gap, leaves 310px for text.
        assert_eq!(handle, Rect::new(20, 40, 314, 28));
        let status_region = Rect::new(364, 40, 44, 28);
        assert!(handle.right() <= status_region.x);
    }

    #[test]
    fn activity_header_rects_cover_dot_and_status_label() {
        let rects = activity_header_rects(1000, Rect::new(10, 20, 400, 200));
        assert_eq!(
            rects,
            vec![Rect::new(20, 20, 11, 28), Rect::new(242, 20, 168, 28)]
        );
        assert!(rects[0].contains(25, 33));
        assert!(rects[1].right() <= 410);
        assert!(activity_header_rects(1000, Rect::new(0, 0, 20, 10)).is_empty());
    }

    #[test]
    fn pane_handle_zone_covers_dot_and_name_only() {
        let metrics = OneCell;
        let slot = Rect::new(8, 40, 400, 200);
        assert_eq!(
            pane_handle_rect(&metrics, 1000, slot, "review", PaneStatus::Quiet, false),
            Some(Rect::new(20, 40, 86, 28))
        );
        assert_eq!(
            pane_handle_rect(&metrics, 1000, slot, "", PaneStatus::Quiet, false),
            Some(Rect::new(20, 40, 14, 28))
        );
        assert!(pane_handle_rect(
            &metrics,
            1000,
            Rect::new(0, 0, 10, 200),
            "x",
            PaneStatus::Quiet,
            false
        )
        .is_none());
    }
}
