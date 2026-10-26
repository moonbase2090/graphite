//! Buffer-snapshot tests: one text snapshot per widget plus token color checks.
//! Run with `UPDATE_SNAPSHOTS=1` to rewrite the files in `tests/snapshots`.

use graphite_core::{sidebar_layout, Tokens};
use graphite_core::{Dot, LegendRun, PaneStatus, Rect as PxRect, SidebarRow, TabText, DARK, LIGHT};
use graphite_tui::{
    legend_layout, picker_layout, tabs_bar_layout, Legend, PaneChrome, Picker, SideRail, Sidebar,
    TabsBar, CELL_H, CELL_W,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;
use std::path::PathBuf;

#[cfg(feature = "cell-design")]
use graphite_tui::{
    BorderStyle, ASCII_FILL, BLOCK_TITLE_INSET, CELL_SPACING, EIGHTH_BLOCKS, SUBCELL_GLYPH_ORDER,
};

fn rgb(c: [u8; 3]) -> Color {
    Color::Rgb(c[0], c[1], c[2])
}

fn text_of(buf: &Buffer) -> String {
    let a = buf.area;
    let mut out = String::new();
    for y in a.y..a.bottom() {
        let row: String = (a.x..a.right()).map(|x| buf[(x, y)].symbol()).collect();
        out.push_str(row.trim_end());
        out.push('\n');
    }
    out
}

fn check(name: &str, buf: &Buffer) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots")
        .join(format!("{name}.txt"));
    let got = text_of(buf);
    if std::env::var("UPDATE_SNAPSHOTS").is_ok_and(|v| !v.is_empty()) {
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(got, want, "snapshot {name} differs");
}

fn tabs() -> Vec<TabText> {
    vec![
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
    ]
}

fn render_tabs(t: &Tokens) -> Buffer {
    let area = Rect::new(0, 0, 90, 3);
    let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
    let mut buf = Buffer::empty(area);
    TabsBar::new(&layout, t).render(area, &mut buf);
    buf
}

#[test]
fn tabs_bar_snapshot() {
    check("tabs_bar", &render_tabs(&DARK));
}

#[test]
fn tabs_bar_colors_come_from_tokens() {
    for t in [&DARK, &LIGHT] {
        let buf = render_tabs(t);
        let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
        let chip = layout.tabs[0].chip;
        let (cx, cy) = ((chip.x / CELL_W + 4) as u16, (chip.y / CELL_H + 1) as u16);
        assert_eq!(buf[(cx, cy)].bg, rgb(t.tab_active));
        assert_eq!(buf[(cx, cy)].fg, rgb(t.text_strong));
        assert_eq!(buf[((chip.x / CELL_W + 1) as u16, cy)].fg, rgb(t.working));
        assert_eq!(buf[(0, 0)].bg, rgb(t.bar));
        assert_eq!(buf[(5, 2)].bg, rgb(t.bar_line));
    }
}

#[test]
fn side_rail_snapshot_and_colors() {
    let area = Rect::new(0, 0, 60, 2);
    let mut buf = Buffer::empty(area);
    SideRail::new(&["main", "docs"], 0, &DARK).render(area, &mut buf);
    check("side_rail", &buf);
    assert_eq!(buf[(0, 1)].bg, rgb(DARK.status_bar));
    assert_eq!(buf[(3, 1)].bg, rgb(DARK.chip_active));
}

fn sidebar_rows() -> Vec<SidebarRow<'static>> {
    let row = |label, depth, selected, mail, needs_you, dot| SidebarRow {
        slot: PxRect::new(0, 0, 0, 0),
        depth,
        chevron: if depth == 0 { Some(true) } else { None },
        dot,
        label,
        mail,
        needs_you,
        selected,
        hovered: false,
    };
    vec![
        row("main", 0, false, 0, 0, Some(Dot::Idle)),
        row("build", 1, true, 3, 0, Some(Dot::Working)),
        row("review", 1, false, 0, 2, Some(Dot::Attention)),
    ]
}

#[test]
fn sidebar_snapshot_and_colors() {
    let area = Rect::new(0, 0, 32, 14);
    let layout = sidebar_layout(1000, PxRect::new(0, 0, 32 * CELL_W, 14 * CELL_H), 3, 0);
    let rows = sidebar_rows();
    let mut buf = Buffer::empty(area);
    Sidebar::new(&layout, &rows, &DARK).render(area, &mut buf);
    check("sidebar", &buf);
    let selected = layout.rows[1];
    let (x, y) = (1, (selected.y / CELL_H) as u16);
    assert_eq!(buf[(x, y)].bg, rgb(DARK.tab_active));
}

#[test]
fn pane_chrome_snapshot_and_colors() {
    let area = Rect::new(0, 0, 40, 6);
    let mut buf = Buffer::empty(area);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, &DARK).render(area, &mut buf);
    check("pane_focused", &buf);
    assert_eq!(buf[(0, 0)].fg, rgb(DARK.title_focus_line));
    assert_eq!(buf[(5, 1)].bg, rgb(DARK.title_focus));

    let mut buf = Buffer::empty(area);
    PaneChrome::new(slot, "agent-2", PaneStatus::Quiet, false, &LIGHT).render(area, &mut buf);
    check("pane_quiet", &buf);
    assert_eq!(buf[(0, 0)].fg, rgb(LIGHT.hairline));
}

#[cfg(feature = "cell-design")]
#[test]
fn pane_chrome_uses_cell_design_border_and_title_tokens() {
    let area = Rect::new(0, 0, 40, 6);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    let mut buf = Buffer::empty(area);
    PaneChrome::new(slot, "agent", PaneStatus::Quiet, false, &DARK)
        .border_style(BorderStyle::Thick)
        .render(area, &mut buf);
    assert_eq!(buf[(0, 0)].symbol(), "┏");
    assert_eq!(buf[(1, 0)].symbol(), "━");
    assert_eq!(buf[(0, 1)].symbol(), "┃");
    assert_eq!(buf[(39, 5)].symbol(), "┛");
    assert_eq!(buf[(4, 1)].symbol(), "a");

    let mut ascii = Buffer::empty(area);
    PaneChrome::new(slot, "agent", PaneStatus::Quiet, false, &DARK)
        .border_style(BorderStyle::Plain)
        .ascii_borders(true)
        .render(area, &mut ascii);
    assert_eq!(ascii[(0, 0)].symbol(), "+");
    assert_eq!(ascii[(1, 0)].symbol(), "-");
    assert_eq!(ascii[(0, 1)].symbol(), "|");
    assert_eq!(ascii[(39, 5)].symbol(), "+");

    assert_eq!(CELL_SPACING, [0, 1, 2, 3, 4, 6]);
    assert_eq!(BLOCK_TITLE_INSET, 1);
    assert_eq!(EIGHTH_BLOCKS[8], '█');
    assert_eq!(ASCII_FILL, '#');
    assert_eq!(SUBCELL_GLYPH_ORDER.len(), 5);
}

#[test]
fn legend_snapshot_and_colors() {
    let runs = vec![
        LegendRun::Keys {
            caps: vec!["Ctrl".into(), "K".into()],
            caption: "commands".into(),
        },
        LegendRun::Text("3 panes".into()),
    ];
    let paint = legend_layout(50, 2, 1, &runs);
    let area = Rect::new(0, 0, 50, 2);
    let mut buf = Buffer::empty(area);
    Legend::new(&paint, &DARK).render(area, &mut buf);
    check("legend", &buf);
    let chip = &paint.chips[0];
    let (x, y) = (
        ((chip.rect.x + CELL_W / 2) / CELL_W) as u16,
        ((chip.rect.y + CELL_H / 2) / CELL_H) as u16,
    );
    assert_eq!(buf[(x, y)].bg, rgb(DARK.key));
}

#[test]
fn picker_snapshot_and_colors() {
    let (cols, rows) = (60u16, 20u16);
    let layout = picker_layout(cols, rows, 4, 0).expect("picker fits");
    let names = ["Graphite Dark", "Graphite Light", "Nord", "Solarized"];
    let area = Rect::new(0, 0, cols, rows);
    let mut buf = Buffer::empty(area);
    Picker::new(&layout, &names, 1, &LIGHT).render(area, &mut buf);
    check("picker", &buf);
    let (px, py) = (
        (layout.panel.x / CELL_W + 3) as u16,
        (layout.panel.y / CELL_H + 4) as u16,
    );
    assert_eq!(buf[(px, py)].bg, rgb(LIGHT.panel));
}

#[test]
fn widgets_clip_to_the_render_area() {
    let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
    let area = Rect::new(3, 2, 20, 3);
    let mut buf = Buffer::empty(Rect::new(0, 0, 30, 6));
    TabsBar::new(&layout, &DARK).render(area, &mut buf);
    assert_eq!(buf[(0, 0)].symbol(), " ");
    assert_eq!(buf[(25, 3)].symbol(), " ");
}

/// Widget sources must take every color from `Tokens` through `style.rs`.
#[test]
fn only_style_rs_names_colors() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.file_name().is_some_and(|n| n == "style.rs") {
            continue;
        }
        let src = std::fs::read_to_string(&path).unwrap();
        let code: String = src.split("#[cfg(test)]").next().unwrap().to_owned();
        for banned in ["Color::", "Rgb(", "Color {", "0x"] {
            assert!(!code.contains(banned), "{} uses {banned}", path.display());
        }
    }
}
