//! Prints every widget as true-color ANSI. Usage: `cargo run -p graphite-tui --example preview [light]`.

use graphite_core::{
    sidebar_layout, Dot, LegendRun, PaneStatus, Rect as PxRect, SidebarRow, TabText, DARK, LIGHT,
};
use graphite_tui::{
    legend_layout, picker_layout, tabs_bar_layout, Legend, PaneChrome, Picker, SideRail, Sidebar,
    TabsBar, CELL_H, CELL_W,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;

#[cfg(feature = "cell-design")]
use graphite_tui::BorderStyle;

fn dump(buf: &Buffer) {
    let a = buf.area;
    for y in a.y..a.bottom() {
        let mut line = String::new();
        for x in a.x..a.right() {
            let c = &buf[(x, y)];
            if let Color::Rgb(r, g, b) = c.fg {
                line += &format!("\x1b[38;2;{r};{g};{b}m");
            }
            if let Color::Rgb(r, g, b) = c.bg {
                line += &format!("\x1b[48;2;{r};{g};{b}m");
            }
            line += c.symbol();
        }
        println!("{line}\x1b[0m");
    }
    println!();
}

fn main() {
    let light = std::env::args().nth(1).as_deref() == Some("light");
    let t = if light { &LIGHT } else { &DARK };
    let cols = 90;

    let tabs = vec![
        TabText {
            label: "build".into(),
            meta: Some("2".into()),
            dot: Dot::Working,
            attention: false,
            selected: true,
        },
        TabText {
            label: "review".into(),
            meta: None,
            dot: Dot::Attention,
            attention: true,
            selected: false,
        },
        TabText {
            label: "notes".into(),
            meta: None,
            dot: Dot::Idle,
            attention: false,
            selected: false,
        },
    ];
    let layout = tabs_bar_layout(cols, "Graphite", &tabs, "Ctrl K");
    let area = Rect::new(0, 0, cols, 3);
    let mut buf = Buffer::empty(area);
    TabsBar::new(&layout, t).render(area, &mut buf);
    dump(&buf);

    let area = Rect::new(0, 0, cols, 2);
    let mut buf = Buffer::empty(area);
    SideRail::new(&["main", "docs"], 0, t).render(area, &mut buf);
    dump(&buf);

    let area = Rect::new(0, 0, 34, 14);
    let layout = sidebar_layout(1000, PxRect::new(0, 0, 34 * CELL_W, 14 * CELL_H), 3, 0);
    let row = |label, depth, selected, mail, needs_you, dot| SidebarRow {
        slot: PxRect::new(0, 0, 0, 0),
        depth,
        chevron: (depth == 0).then_some(true),
        dot: Some(dot),
        label,
        mail,
        needs_you,
        selected,
        hovered: false,
    };
    let rows = [
        row("main", 0, false, 0, 0, Dot::Idle),
        row("build", 1, true, 3, 0, Dot::Working),
        row("review", 1, false, 0, 2, Dot::Attention),
    ];
    let mut buf = Buffer::empty(area);
    Sidebar::new(&layout, &rows, t).render(area, &mut buf);
    dump(&buf);

    let area = Rect::new(0, 0, 40, 6);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    #[cfg(feature = "cell-design")]
    for (border_style, ascii) in [
        (BorderStyle::Rounded, false),
        (BorderStyle::Plain, false),
        (BorderStyle::Thick, false),
        (BorderStyle::Rounded, true),
    ] {
        println!("PaneChrome border={border_style:?} ascii={ascii}");
        let mut buf = Buffer::empty(area);
        PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, t)
            .border_style(border_style)
            .ascii_borders(ascii)
            .render(area, &mut buf);
        dump(&buf);
    }
    #[cfg(not(feature = "cell-design"))]
    {
        let mut buf = Buffer::empty(area);
        PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, t).render(area, &mut buf);
        dump(&buf);
    }

    let runs = [
        LegendRun::Keys {
            caps: vec!["Ctrl".into(), "K".into()],
            caption: "commands".into(),
        },
        LegendRun::Text("3 panes".into()),
    ];
    let paint = legend_layout(50, 2, 1, &runs);
    let area = Rect::new(0, 0, 50, 2);
    let mut buf = Buffer::empty(area);
    Legend::new(&paint, t).render(area, &mut buf);
    dump(&buf);

    let layout = picker_layout(60, 20, 4, 0).expect("picker fits");
    let area = Rect::new(0, 0, 60, 20);
    let mut buf = Buffer::empty(area);
    Picker::new(
        &layout,
        &["Graphite Dark", "Graphite Light", "Nord", "Solarized"],
        1,
        t,
    )
    .render(area, &mut buf);
    dump(&buf);
}
