// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Prints every graphite-lunatui widget in one scene as terminal text.
//!
//! Usage: `cargo run --manifest-path graphite-lunatui/Cargo.toml --example gallery [dark|light]`

use graphite_core::{
    sidebar_layout, Dot, LegendRun, PaneStatus, Rect as PxRect, SidebarRow, TabText, Tokens, DARK,
    LIGHT,
};
use graphite_lunatui::{
    legend_layout, picker_layout, tabs_bar_layout, Badge, Chip, Legend, PaneChrome, Picker,
    SideRail, Sidebar, StatusDot, TabsBar, CELL_H, CELL_W,
};
use lunatui_core::{ClusterTable, Grid, Rect};
use lunatui_theme::{Theme, Variant};
use lunatui_widgets::{RenderCx, Widget};

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

fn side_rows() -> Vec<SidebarRow<'static>> {
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

fn scene(tokens: &Tokens) -> String {
    let theme = Theme::builtin(Variant::Dark);
    let mut grid = Grid::new(96, 34);
    let mut clusters = ClusterTable::new();
    let mut cx = RenderCx::new(&mut grid, &mut clusters, &theme);
    let layout = tabs_bar_layout(96, "Graphite", &tabs(), "Ctrl K");
    TabsBar::new(&layout, tokens).render(Rect::new(0, 0, 96, 3), &mut cx);
    let side = sidebar_layout(
        1000,
        PxRect::new(0, 3 * CELL_H, 32 * CELL_W, 14 * CELL_H),
        3,
        0,
    );
    Sidebar::new(&side, &side_rows(), tokens).render(Rect::new(0, 3, 32, 14), &mut cx);
    let slot = PxRect::new(34 * CELL_W, 3 * CELL_H, 40 * CELL_W, 6 * CELL_H);
    PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, tokens)
        .render(Rect::new(0, 3, 96, 14), &mut cx);
    SideRail::new(&["main", "docs"], 0, tokens).render(Rect::new(34, 10, 60, 2), &mut cx);
    StatusDot::new(Dot::Attention, tokens).render(Rect::new(34, 13, 1, 1), &mut cx);
    Chip::new("build", Some(Dot::Working), true, tokens).render(Rect::new(36, 13, 9, 1), &mut cx);
    Badge::new(3, Dot::Attention, tokens).render(Rect::new(46, 13, 3, 1), &mut cx);
    let runs = vec![
        LegendRun::Keys {
            caps: vec!["Ctrl".into(), "K".into()],
            caption: "commands".into(),
        },
        LegendRun::Text("3 panes".into()),
    ];
    let paint = legend_layout(50, 2, 1, &runs);
    Legend::new(&paint, tokens).render(Rect::new(0, 18, 50, 2), &mut cx);
    if let Some(picker) = picker_layout(60, 20, 4, 0) {
        Picker::new(
            &picker,
            &["Graphite Dark", "Graphite Light", "Nord", "Solarized"],
            1,
            tokens,
        )
        .render(Rect::new(34, 14, 60, 20), &mut cx);
    }
    grid.display_text(&clusters)
}

fn main() {
    let arg = std::env::args().nth(1);
    let tokens = if arg.as_deref() == Some("light") {
        &LIGHT
    } else {
        &DARK
    };
    print!("{}", scene(tokens));
}
