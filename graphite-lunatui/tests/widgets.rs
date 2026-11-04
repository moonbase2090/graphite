// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Cell-grid snapshot tests: every widget renders into a lunatui grid and the
//! exact cells (glyphs and token colors) are asserted. The tabs-bar fixture
//! mirrors the Ratatui crate's snapshot so both backends stay in parity.

use graphite_core::{Dot, TabText, Tokens, DARK, LIGHT};
use graphite_lunatui::{tabs_bar_layout, Badge, Chip, StatusDot, TabsBar};
use lunatui_core::{Cell, ClusterTable, Color, Grid, Rect, Size, Style};
use lunatui_theme::{Theme, Variant};
use lunatui_widgets::{RenderCx, Widget};

fn draw(widget: &dyn Widget, area: Rect) -> (Grid, ClusterTable) {
    let theme = Theme::builtin(Variant::Dark);
    let mut grid = Grid::new(area.x + area.width, area.y + area.height);
    let mut clusters = ClusterTable::new();
    let mut cx = RenderCx::new(&mut grid, &mut clusters, &theme);
    widget.render(area, &mut cx);
    (grid, clusters)
}

fn rows(widget: &dyn Widget, area: Rect) -> Vec<String> {
    let (grid, clusters) = draw(widget, area);
    (area.y..area.y + area.height)
        .map(|y| grid.row_display(y, &clusters))
        .collect()
}

fn style_at(grid: &Grid, x: u16, y: u16) -> Style {
    grid.get(x, y).map(Cell::style).unwrap_or_default()
}

fn rgb(c: [u8; 3]) -> Color {
    Color::Rgb(c[0], c[1], c[2])
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

#[test]
fn tabs_bar_matches_the_ratatui_snapshot() {
    let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
    let rendered = rows(&TabsBar::new(&layout, &DARK), Rect::new(0, 0, 90, 3));
    let trimmed: Vec<&str> = rendered.iter().map(|row| row.trim_end()).collect();
    assert_eq!(
        trimmed,
        [
            "",
            "  Graphite ▾     │ ● build 2  ×   ● review  needs you  ×  +",
            "",
        ]
    );
}

#[test]
fn tabs_bar_colors_come_from_tokens() {
    for t in [&DARK, &LIGHT] {
        let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
        let (grid, clusters) = draw(&TabsBar::new(&layout, t), Rect::new(0, 0, 90, 3));
        // Bar background and the selected chip's fill are token colors; the
        // working dot is the working token, not a hard-coded color.
        let dot_x = u16::try_from(
            grid.row_display(1, &clusters)
                .chars()
                .position(|c| c == '●')
                .expect("dot drawn"),
        )
        .unwrap_or(0);
        assert_eq!(style_at(&grid, 0, 0).bg, rgb(t.bar));
        assert_eq!(style_at(&grid, dot_x, 1).bg, rgb(t.tab_active));
        assert_eq!(style_at(&grid, dot_x, 1).fg, rgb(t.working));
        // The bottom row is the hairline rule in the bar-line token.
        assert_eq!(style_at(&grid, 0, 2).bg, rgb(t.bar_line));
    }
}

#[test]
fn tabs_bar_clears_cells_it_no_longer_draws() {
    let theme = Theme::builtin(Variant::Dark);
    let mut grid = Grid::new(90, 3);
    let mut clusters = ClusterTable::new();
    grid.fill(Rect::new(0, 0, 90, 3), Style::new(), 'X', &mut clusters);
    let layout = tabs_bar_layout(90, "Graphite", &tabs(), "Ctrl K");
    {
        let mut cx = RenderCx::new(&mut grid, &mut clusters, &theme);
        TabsBar::new(&layout, &DARK).render(Rect::new(0, 0, 90, 3), &mut cx);
    }
    for y in 0..3 {
        assert!(
            !grid.row_display(y, &clusters).contains('X'),
            "row {y} kept a stale glyph"
        );
    }
}

#[test]
fn status_dot_draws_the_dot_token_color() {
    for (dot, want) in [
        (Dot::Working, DARK.working),
        (Dot::Unseen, DARK.unseen),
        (Dot::Attention, DARK.attention),
        (Dot::Idle, DARK.idle),
    ] {
        let widget = StatusDot::new(dot, &DARK);
        let (grid, clusters) = draw(&widget, Rect::new(0, 0, 1, 1));
        assert_eq!(grid.row_display(0, &clusters), "●");
        assert_eq!(style_at(&grid, 0, 0).fg, rgb(want));
        assert_eq!(widget.measure(Size::new(9, 9)), Size::new(1, 1));
    }
}

#[test]
fn chip_draws_dot_label_and_selection_fill() {
    let selected = Chip::new("build", Some(Dot::Working), true, &DARK);
    let (grid, clusters) = draw(&selected, Rect::new(0, 0, 10, 1));
    assert_eq!(grid.row_display(0, &clusters), " ● build  ");
    assert_eq!(style_at(&grid, 0, 0).bg, rgb(DARK.tab_active));
    assert_eq!(style_at(&grid, 1, 0).fg, rgb(DARK.working));
    assert_eq!(
        selected.measure(Size::new(80, 4)),
        Size::new(9, 1),
        "dot, label and padding"
    );
    let plain = Chip::new("docs", None, false, &DARK);
    assert_eq!(rows(&plain, Rect::new(0, 0, 8, 1)), [" docs   "]);
    let (grid, _) = draw(&plain, Rect::new(0, 0, 8, 1));
    assert_eq!(style_at(&grid, 0, 0).bg, rgb(DARK.bar));
    // A label wider than the area is clipped, never wrapped or spilled.
    let wide = Chip::new("documentation", None, false, &DARK);
    assert_eq!(rows(&wide, Rect::new(0, 0, 5, 1)), [" docu"]);
}

#[test]
fn badge_draws_a_count_pill_and_hides_at_zero() {
    let badge = Badge::new(3, Dot::Attention, &DARK);
    let (grid, clusters) = draw(&badge, Rect::new(0, 0, 5, 1));
    assert_eq!(grid.row_display(0, &clusters), " 3   ");
    assert_eq!(style_at(&grid, 0, 0).bg, rgb(DARK.attention));
    assert_eq!(style_at(&grid, 1, 0).fg, rgb(DARK.on_attention));
    assert_eq!(badge.measure(Size::new(80, 4)), Size::new(3, 1));
    let big = Badge::new(128, Dot::Unseen, &DARK);
    assert_eq!(big.measure(Size::new(80, 4)), Size::new(5, 1));
    let zero = Badge::new(0, Dot::Attention, &DARK);
    assert_eq!(zero.measure(Size::new(80, 4)), Size::new(0, 0));
    assert_eq!(rows(&zero, Rect::new(0, 0, 4, 1)), ["    "]);
}

#[test]
fn tokens_drive_dark_and_light_chips() {
    let dark = Chip::new("x", None, true, &DARK);
    let light = Chip::new("x", None, true, &LIGHT);
    let (grid, _) = draw(&dark, Rect::new(0, 0, 3, 1));
    assert_eq!(style_at(&grid, 0, 0).bg, rgb(DARK.tab_active));
    let (grid, _) = draw(&light, Rect::new(0, 0, 3, 1));
    assert_eq!(style_at(&grid, 0, 0).bg, rgb(LIGHT.tab_active));
    let tokens: &Tokens = &DARK;
    assert_ne!(tokens.tab_active, LIGHT.tab_active);
}

#[test]
fn pane_chrome_matches_the_ratatui_snapshots() {
    use graphite_core::{PaneStatus, Rect as PxRect};
    use graphite_lunatui::{PaneChrome, CELL_H, CELL_W};
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    let focused = PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, &DARK);
    let trimmed: Vec<String> = rows(&focused, Rect::new(0, 0, 40, 6))
        .iter()
        .map(|row| row.trim_end().to_string())
        .collect();
    assert_eq!(
        trimmed,
        [
            "╭──────────────────────────────────────╮",
            "│ ● agent-1                     2 mail │",
            "│                                      │",
            "│                                      │",
            "│                                      │",
            "╰──────────────────────────────────────╯",
        ]
    );
    let (grid, _) = draw(&focused, Rect::new(0, 0, 40, 6));
    assert_eq!(style_at(&grid, 0, 0).fg, rgb(DARK.title_focus_line));
    assert_eq!(style_at(&grid, 5, 1).bg, rgb(DARK.title_focus));
    let quiet = PaneChrome::new(slot, "agent-2", PaneStatus::Quiet, false, &LIGHT);
    let (grid, clusters) = draw(&quiet, Rect::new(0, 0, 40, 6));
    assert_eq!(
        grid.row_display(1, &clusters).trim_end(),
        "│ ● agent-2                            │"
    );
    assert_eq!(style_at(&grid, 0, 0).fg, rgb(LIGHT.hairline));
}

#[test]
fn side_rail_matches_the_ratatui_snapshot() {
    use graphite_lunatui::SideRail;
    let rail = SideRail::new(&["main", "docs"], 0, &DARK);
    let trimmed: Vec<String> = rows(&rail, Rect::new(0, 0, 60, 2))
        .iter()
        .map(|row| row.trim_end().to_string())
        .collect();
    assert_eq!(trimmed, ["", "  ● main×   docs ×   + New space"]);
    let (grid, _) = draw(&rail, Rect::new(0, 0, 60, 2));
    assert_eq!(style_at(&grid, 0, 1).bg, rgb(DARK.status_bar));
    assert_eq!(style_at(&grid, 3, 1).bg, rgb(DARK.chip_active));
}
