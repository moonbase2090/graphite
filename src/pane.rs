//! Renderer-independent pane status and header geometry.

use crate::{Dot, Face, Rect, TextMetrics};

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
        let _ = (attention, mail, unseen, running, focused);
        todo!()
    }
}

/// Returns the measured width of a pane-header status label.
pub fn status_width<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    status: PaneStatus,
) -> f32 {
    let _ = (metrics, scale_milli, status);
    todo!()
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
    let _ = (metrics, scale_milli, slot, name, status, focus_row);
    todo!()
}

/// Returns damage rectangles for the pane-header activity regions.
pub fn activity_header_rects(scale_milli: u32, slot: Rect) -> Vec<Rect> {
    let _ = (scale_milli, slot);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
