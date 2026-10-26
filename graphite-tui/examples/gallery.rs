//! Prints every widget as ANSI text.
//!
//! Usage: `cargo run -p graphite-tui --example gallery [dark|light] [truecolor|256|16]`,
//! or `gallery all` for dark and light at every depth.

use graphite_core::{
    sidebar_layout, terminal_palette, xterm256, Ansi16, Dot, LegendRun, PaneStatus, Rect as PxRect,
    SidebarRow, TabText, TerminalToken, Tokens, DARK, LIGHT,
};
use graphite_tui::{
    legend_layout, picker_layout, tabs_bar_layout, Legend, PaneChrome, Picker, SideRail, Sidebar,
    TabsBar, CELL_H, CELL_W,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;

#[derive(Clone, Copy, PartialEq)]
enum Depth {
    TrueColor,
    Ansi256,
    Ansi16,
}

/// Maps a token color to the SGR parameters for the chosen depth.
fn sgr(c: Color, depth: Depth, fg: bool, table: &[TerminalToken]) -> Option<String> {
    let Color::Rgb(r, g, b) = c else {
        return None;
    };
    let base = if fg { 38 } else { 48 };
    Some(match depth {
        Depth::TrueColor => format!("{base};2;{r};{g};{b}"),
        Depth::Ansi256 => format!("{base};5;{}", xterm256([r, g, b])),
        Depth::Ansi16 => {
            // Every widget color is a token, so the semantic slot is an exact lookup.
            let slot = table
                .iter()
                .find(|t| t.rgb == [r, g, b])
                .map_or(Ansi16::White as u8, |t| t.ansi16 as u8);
            let code = if slot < 8 { 30 + slot } else { 90 + slot - 8 };
            format!("{}", if fg { code } else { code + 10 })
        }
    })
}

fn dump(buf: &Buffer, depth: Depth, table: &[TerminalToken]) {
    let a = buf.area;
    for y in a.y..a.bottom() {
        let mut line = String::new();
        for x in a.x..a.right() {
            let c = &buf[(x, y)];
            for (color, fg) in [(c.fg, true), (c.bg, false)] {
                if let Some(code) = sgr(color, depth, fg, table) {
                    line += &format!("\x1b[{code}m");
                }
            }
            line += c.symbol();
        }
        println!("{line}\x1b[0m");
    }
    println!();
}

fn main() {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    if first.as_deref() == Some("all") {
        for (name, tokens) in [("dark", &DARK), ("light", &LIGHT)] {
            for (label, depth) in [
                ("truecolor", Depth::TrueColor),
                ("256", Depth::Ansi256),
                ("16", Depth::Ansi16),
            ] {
                println!("== {name} {label}");
                gallery(tokens, depth);
            }
        }
        return;
    }
    let tokens = if first.as_deref() == Some("light") {
        &LIGHT
    } else {
        &DARK
    };
    let depth = match args.next().as_deref() {
        Some("256") => Depth::Ansi256,
        Some("16") => Depth::Ansi16,
        _ => Depth::TrueColor,
    };
    gallery(tokens, depth);
}

fn gallery(t: &Tokens, depth: Depth) {
    let table = terminal_palette(t);
    let table = table.as_slice();
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
    dump(&buf, depth, table);

    let area = Rect::new(0, 0, cols, 2);
    let mut buf = Buffer::empty(area);
    SideRail::new(&["main", "docs"], 0, t).render(area, &mut buf);
    dump(&buf, depth, table);

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
    dump(&buf, depth, table);

    let area = Rect::new(0, 0, 40, 6);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    let mut buf = Buffer::empty(area);
    PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, t).render(area, &mut buf);
    dump(&buf, depth, table);

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
    dump(&buf, depth, table);

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
    dump(&buf, depth, table);
}
