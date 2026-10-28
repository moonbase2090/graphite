//! Rendering must not allocate: layouts and labels are prepared before `render`.

use graphite_core::{
    sidebar_layout, Dot, LegendRun, PaneStatus, Rect as PxRect, SidebarRow, TabText, DARK, LIGHT,
};
use graphite_tui::{
    legend_layout, picker_layout, tabs_bar_layout, Legend, PaneChrome, Picker, SideRail, Sidebar,
    TabsBar, CELL_H, CELL_W,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use std::alloc::{GlobalAlloc, Layout, System};

struct Counting;

// Counts per thread: the harness and other threads may allocate while the
// measured closure runs, and those allocations must not be attributed to it.
thread_local! {
    static COUNTING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = COUNTING.try_with(|on| {
            if on.get() {
                let _ = COUNT.try_with(|n| n.set(n.get() + 1));
            }
        });
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Counts allocations made by `f` on the calling thread only.
fn allocations(f: impl FnOnce()) -> usize {
    COUNT.with(|n| n.set(0));
    COUNTING.with(|on| on.set(true));
    f();
    COUNTING.with(|on| on.set(false));
    COUNT.with(|n| n.get())
}

#[test]
fn render_allocates_nothing_for_every_widget() {
    // Everything is prepared up front; this is a single test so no other
    // thread allocates while the counter is active.
    let tabs = vec![
        TabText {
            label: "build".into(),
            meta: Some("2".into()),
            dot: Dot::Working,
            attention: true,
            selected: true,
        },
        TabText {
            label: "review".into(),
            meta: None,
            dot: Dot::Idle,
            attention: false,
            selected: false,
        },
    ];
    let bar = tabs_bar_layout(90, "Graphite", &tabs, "Ctrl K");
    let bar_area = Rect::new(0, 0, 90, 3);
    let mut bar_buf = Buffer::empty(bar_area);

    let rail_area = Rect::new(0, 0, 60, 2);
    let mut rail_buf = Buffer::empty(rail_area);

    let side_area = Rect::new(0, 0, 32, 14);
    let side = sidebar_layout(1000, PxRect::new(0, 0, 32 * CELL_W, 14 * CELL_H), 3, 0);
    let row = |label, mail, needs_you| SidebarRow {
        slot: PxRect::new(0, 0, 0, 0),
        depth: 1,
        chevron: Some(true),
        dot: Some(Dot::Attention),
        label,
        mail,
        needs_you,
        selected: false,
        hovered: true,
    };
    let rows = [row("main", 0, 0), row("build", 3, 0), row("review", 0, 2)];
    let mut side_buf = Buffer::empty(side_area);

    let pane_area = Rect::new(0, 0, 40, 6);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    let mut pane_buf = Buffer::empty(pane_area);

    let runs = [LegendRun::Keys {
        caps: vec!["Ctrl".into(), "K".into()],
        caption: "commands".into(),
    }];
    let legend = legend_layout(50, 2, 1, &runs);
    let legend_area = Rect::new(0, 0, 50, 2);
    let mut legend_buf = Buffer::empty(legend_area);

    let picker = picker_layout(60, 20, 4, 0).expect("picker fits");
    let names = ["Dark", "Light", "Nord", "Solarized"];
    let picker_area = Rect::new(0, 0, 60, 20);
    let mut picker_buf = Buffer::empty(picker_area);

    let count = |what: &str, n: usize| assert_eq!(n, 0, "{what} allocated {n} time(s)");
    for t in [&DARK, &LIGHT] {
        count(
            "TabsBar",
            allocations(|| TabsBar::new(&bar, t).render(bar_area, &mut bar_buf)),
        );
        count(
            "SideRail",
            allocations(|| SideRail::new(&["main", "docs"], 0, t).render(rail_area, &mut rail_buf)),
        );
        count(
            "Sidebar",
            allocations(|| Sidebar::new(&side, &rows, t).render(side_area, &mut side_buf)),
        );
        for status in [
            PaneStatus::Mail(12),
            PaneStatus::Attention,
            PaneStatus::Running,
            PaneStatus::Quiet,
        ] {
            count(
                "PaneChrome",
                allocations(|| {
                    PaneChrome::new(slot, "build", status, true, t).render(pane_area, &mut pane_buf)
                }),
            );
        }
        count(
            "Legend",
            allocations(|| Legend::new(&legend, t).render(legend_area, &mut legend_buf)),
        );
        count(
            "Picker",
            allocations(|| Picker::new(&picker, &names, 1, t).render(picker_area, &mut picker_buf)),
        );
    }
}
