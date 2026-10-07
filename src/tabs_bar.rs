//! Renderer-independent tabs-bar layout and hit testing.

use crate::{Face, Rect, TextMetrics};

/// Status dot shown in a tab chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dot {
    /// The tab needs attention.
    Attention,
    /// The tab has active output.
    Working,
    /// The tab has unseen output.
    Unseen,
    /// The tab is quiet.
    Idle,
}

impl Dot {
    /// Selects the highest-priority status for a tab.
    pub fn for_tab(attention: bool, active: bool, unseen: bool) -> Self {
        if attention {
            Self::Attention
        } else if active {
            Self::Working
        } else if unseen {
            Self::Unseen
        } else {
            Self::Idle
        }
    }
}

/// Text and status data supplied by a Graphite host for one tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabText {
    /// The tab's primary label.
    pub label: String,
    /// Optional muted metadata after the label.
    pub meta: Option<String>,
    /// The status dot to display.
    pub dot: Dot,
    /// Whether to display the `needs you` pill.
    pub attention: bool,
    /// Whether this tab is selected.
    pub selected: bool,
}

/// A tab's laid-out chip and close target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabSlot {
    /// The complete chip rectangle.
    pub chip: Rect,
    /// The close-button rectangle inside the chip.
    pub close: Rect,
    /// The ellipsized label displayed in the chip.
    pub label: String,
    /// The original metadata displayed in the chip.
    pub meta: Option<String>,
    /// The status dot to paint.
    pub dot: Dot,
    /// Whether to paint the attention pill.
    pub attention: bool,
    /// Whether the tab is selected.
    pub selected: bool,
}

/// The core hit target returned by tabs-bar hit testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarHit {
    /// The space selector dropdown.
    SpaceMenu,
    /// A tab chip, optionally on its close target.
    Tab { index: usize, close: bool },
    /// The new-tab button.
    NewTab,
    /// The command field.
    Command,
    /// Empty space after the plus button.
    EmptyEnd,
}

/// A drop-highlight target supplied by the host while dragging a pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropTarget {
    /// A tab chip by its zero-based index.
    Tab(usize),
    /// The new-tab slot after the plus button.
    NewTab,
}

/// The complete tabs-bar layout in window pixels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarLayout {
    /// The complete bar rectangle.
    pub bar: Rect,
    /// The space selector rectangle.
    pub dropdown: Rect,
    /// The ellipsized space label.
    pub space_label: String,
    /// The divider's x coordinate.
    pub divider_x: usize,
    /// Tab slots that fit, in input order.
    pub tabs: Vec<TabSlot>,
    /// The new-tab button rectangle.
    pub plus: Rect,
    /// The optional command field rectangle.
    pub command: Option<Rect>,
    /// The host-supplied command chord.
    pub chord: String,
    /// The milli-scale used to produce this layout.
    pub scale_milli: u32,
}

/// Lays out the tabs bar across `width` pixels at row `y`.
pub fn bar_layout<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    width: usize,
    y: usize,
    space_label: &str,
    tabs: &[TabText],
    chord: &str,
) -> BarLayout {
    let _ = (metrics, scale_milli, width, y, space_label, tabs, chord);
    todo!()
}

/// Shifts every hit-testable part of a layout horizontally by `dx` pixels.
pub fn shift_bar(mut layout: BarLayout, dx: usize) -> BarLayout {
    let _ = (&mut layout, dx);
    todo!()
}

/// Returns the core hit target under a window pixel.
pub fn bar_hit(layout: &BarLayout, px: usize, py: usize, reserve_end: bool) -> Option<BarHit> {
    let _ = (layout, px, py, reserve_end);
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

    fn tab(label: &str, selected: bool) -> TabText {
        TabText {
            label: label.to_owned(),
            meta: None,
            dot: Dot::for_tab(false, selected, false),
            attention: false,
            selected,
        }
    }

    #[test]
    fn layout_and_hit_test_agree() {
        let tabs = [tab("grid", true), tab("review", false), tab("notes", false)];
        let layout = bar_layout(
            &OneCell,
            1000,
            1440,
            0,
            "release-lab",
            &tabs,
            "Ctrl Shift P",
        );
        assert_eq!(layout.scale_milli, 1000);
        assert_eq!(layout.bar, Rect::new(0, 0, 1440, 44));
        assert_eq!(layout.dropdown, Rect::new(10, 7, 203, 30));
        assert_eq!(layout.space_label, "release-lab");
        assert_eq!(layout.divider_x, 219);
        assert_eq!(layout.tabs.len(), 3);
        assert_eq!(layout.tabs[0].chip, Rect::new(226, 7, 107, 30));
        assert_eq!(layout.tabs[0].close, Rect::new(313, 7, 16, 30));
        assert_eq!(layout.tabs[1].chip, Rect::new(337, 7, 133, 30));
        assert_eq!(layout.tabs[1].close, Rect::new(450, 7, 16, 30));
        assert_eq!(layout.tabs[2].chip, Rect::new(474, 7, 120, 30));
        assert_eq!(layout.tabs[2].close, Rect::new(574, 7, 16, 30));
        assert_eq!(layout.plus, Rect::new(598, 7, 30, 30));
        assert_eq!(layout.command, Some(Rect::new(1170, 7, 260, 30)));
        assert_eq!(bar_hit(&layout, 111, 22, false), Some(BarHit::SpaceMenu));
        assert_eq!(
            bar_hit(&layout, 230, 22, false),
            Some(BarHit::Tab {
                index: 0,
                close: false
            })
        );
        assert_eq!(
            bar_hit(&layout, 341, 22, false),
            Some(BarHit::Tab {
                index: 1,
                close: false
            })
        );
        assert_eq!(
            bar_hit(&layout, 478, 22, false),
            Some(BarHit::Tab {
                index: 2,
                close: false
            })
        );
        assert_eq!(
            bar_hit(&layout, 321, 22, false),
            Some(BarHit::Tab {
                index: 0,
                close: true
            })
        );
        assert_eq!(
            bar_hit(&layout, 458, 22, false),
            Some(BarHit::Tab {
                index: 1,
                close: true
            })
        );
        assert_eq!(
            bar_hit(&layout, 582, 22, false),
            Some(BarHit::Tab {
                index: 2,
                close: true
            })
        );
        assert_eq!(bar_hit(&layout, 613, 22, false), Some(BarHit::NewTab));
        assert_eq!(bar_hit(&layout, 1300, 22, false), Some(BarHit::Command));
        assert_eq!(bar_hit(&layout, 5, 60, false), None);
    }

    #[test]
    fn narrow_bar_drops_the_command_field_then_shortens_labels() {
        let labels: Vec<String> = (0..6)
            .map(|i| format!("a-very-long-session-name-{i}"))
            .collect();
        let tabs: Vec<TabText> = labels.iter().map(|label| tab(label, false)).collect();
        let layout = bar_layout(&OneCell, 1000, 900, 0, "release-lab", &tabs, "Ctrl Shift P");
        assert_eq!(layout.command, None);
        assert_eq!(layout.tabs.len(), 6);
        for (index, slot) in layout.tabs.iter().enumerate() {
            assert_eq!(slot.label, "a-…", "tab {index}");
            assert_eq!(slot.chip, Rect::new(226 + index * 98, 7, 94, 30));
        }
        assert_eq!(layout.plus, Rect::new(814, 7, 30, 30));
        assert!(layout.plus.right() <= 900);
    }

    #[test]
    fn retina_layout_doubles_the_bar() {
        let tabs = [tab("grid", true)];
        let one = bar_layout(&OneCell, 1000, 1440, 0, "lab", &tabs, "Ctrl Shift P");
        let two = bar_layout(&OneCell, 2000, 2880, 0, "lab", &tabs, "Ctrl Shift P");
        assert_eq!(one.bar, Rect::new(0, 0, 1440, 44));
        assert_eq!(one.dropdown, Rect::new(10, 7, 99, 30));
        assert_eq!(one.divider_x, 115);
        assert_eq!(one.tabs[0].chip, Rect::new(122, 7, 107, 30));
        assert_eq!(one.plus, Rect::new(233, 7, 30, 30));
        assert_eq!(one.command, Some(Rect::new(1170, 7, 260, 30)));
        assert_eq!(two.bar, Rect::new(0, 0, 2880, 88));
        assert_eq!(two.dropdown, Rect::new(20, 14, 198, 60));
        assert_eq!(two.divider_x, 230);
        assert_eq!(two.tabs[0].chip, Rect::new(243, 14, 214, 60));
        assert_eq!(two.tabs[0].close, Rect::new(417, 14, 32, 60));
        assert_eq!(two.plus, Rect::new(465, 14, 60, 60));
        assert_eq!(two.command, Some(Rect::new(2340, 14, 520, 60)));
    }

    #[test]
    fn shift_bar_keeps_hits_on_the_moved_chips() {
        let tabs = [tab("grid", true)];
        let layout = shift_bar(bar_layout(&OneCell, 1000, 800, 0, "lab", &tabs, ""), 220);
        assert_eq!(layout.bar, Rect::new(220, 0, 800, 44));
        assert_eq!(layout.dropdown, Rect::new(230, 7, 99, 30));
        assert_eq!(layout.divider_x, 335);
        assert_eq!(layout.tabs[0].chip, Rect::new(342, 7, 107, 30));
        assert_eq!(layout.tabs[0].close, Rect::new(429, 7, 16, 30));
        assert_eq!(layout.plus, Rect::new(453, 7, 30, 30));
        assert_eq!(layout.command, Some(Rect::new(750, 7, 260, 30)));
        assert_eq!(
            bar_hit(&layout, 346, 22, false),
            Some(BarHit::Tab {
                index: 0,
                close: false
            })
        );
        assert_eq!(bar_hit(&layout, 10, 22, false), None);
    }

    #[test]
    fn drop_target_carries_tab_or_new_tab_semantics_without_painting() {
        let targets = [DropTarget::Tab(1), DropTarget::NewTab];
        assert!(matches!(targets[0], DropTarget::Tab(1)));
        assert!(matches!(targets[1], DropTarget::NewTab));
    }
}
