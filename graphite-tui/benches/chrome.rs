//! Compare frozen Graphite chrome with equivalent styled ratatui paragraphs.
//! All layout, strings, grouping, allocation and correctness checks precede timing.
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graphite_core::{
    sidebar_layout, Dot, LegendRun, PaneStatus, Rect as PxRect, SidebarRow, TabText, DARK, LIGHT,
};
use graphite_tui::{
    legend_layout, picker_layout, tabs_bar_layout, Legend, PaneChrome, Picker, SideRail, Sidebar,
    TabsBar, CELL_H, CELL_W,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};
use std::time::Duration;

type Draw = Box<dyn Fn(&mut Buffer)>;
struct Scene {
    name: &'static str,
    area: Rect,
    draw: Draw,
    plain: Paragraph<'static>,
}

fn scene(name: &'static str, area: Rect, expected: &str, draw: Draw) -> Scene {
    let mut reference = Buffer::empty(area);
    draw(&mut reference);
    let mut lines = Vec::new();
    let mut visible = String::new();
    for y in 0..area.height {
        let row: String = (0..area.width)
            .map(|x| reference[(x, y)].symbol())
            .collect();
        visible.push_str(row.trim_end());
        visible.push('\n');
        let mut spans = Vec::new();
        let mut x = 0;
        while x < area.width {
            let style = reference[(x, y)].style();
            let mut text = String::new();
            while x < area.width && reference[(x, y)].style() == style {
                text.push_str(reference[(x, y)].symbol());
                x += 1;
            }
            spans.push(Span::styled(text, style));
        }
        lines.push(Line::from(spans));
    }
    assert_eq!(
        visible, expected,
        "{name} differs from reviewed text snapshot"
    );
    let plain = Paragraph::new(Text::from(lines));
    let mut control = Buffer::empty(area);
    (&plain).render(area, &mut control);
    assert_eq!(
        control, reference,
        "{name} control differs in symbols or styles"
    );
    eprintln!(
        "verified {name}: {} cells, exact symbols/styles, setup outside timing",
        area.width as usize * area.height as usize
    );
    Scene {
        name,
        area,
        draw,
        plain,
    }
}

fn scenes() -> Vec<Scene> {
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
    ];
    let layout = tabs_bar_layout(90, "Graphite", &tabs, "Ctrl K");
    let tabs = scene(
        "tabs",
        Rect::new(0, 0, 90, 3),
        include_str!("../tests/snapshots/tabs_bar.txt"),
        Box::new(move |buf| TabsBar::new(&layout, &DARK).render(buf.area, buf)),
    );
    let rail = scene(
        "side_rail",
        Rect::new(0, 0, 60, 2),
        include_str!("../tests/snapshots/side_rail.txt"),
        Box::new(|buf| SideRail::new(&["main", "docs"], 0, &DARK).render(buf.area, buf)),
    );
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
    let rows = vec![
        row("main", 0, false, 0, 0, Some(Dot::Idle)),
        row("build", 1, true, 3, 0, Some(Dot::Working)),
        row("review", 1, false, 0, 2, Some(Dot::Attention)),
    ];
    let layout = sidebar_layout(1000, PxRect::new(0, 0, 32 * CELL_W, 14 * CELL_H), 3, 0);
    let sidebar = scene(
        "sidebar",
        Rect::new(0, 0, 32, 14),
        include_str!("../tests/snapshots/sidebar.txt"),
        Box::new(move |buf| Sidebar::new(&layout, &rows, &DARK).render(buf.area, buf)),
    );
    let pane = scene(
        "pane",
        Rect::new(0, 0, 40, 6),
        include_str!("../tests/snapshots/pane_focused.txt"),
        Box::new(|buf| {
            PaneChrome::new(
                PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H),
                "agent-1",
                PaneStatus::Mail(2),
                true,
                &DARK,
            )
            .render(buf.area, buf)
        }),
    );
    let runs = vec![
        LegendRun::Keys {
            caps: vec!["Ctrl".into(), "K".into()],
            caption: "commands".into(),
        },
        LegendRun::Text("3 panes".into()),
    ];
    let paint = legend_layout(50, 2, 1, &runs);
    let legend = scene(
        "legend",
        Rect::new(0, 0, 50, 2),
        include_str!("../tests/snapshots/legend.txt"),
        Box::new(move |buf| Legend::new(&paint, &DARK).render(buf.area, buf)),
    );
    let layout = picker_layout(60, 20, 4, 0).unwrap();
    let picker = scene(
        "picker",
        Rect::new(0, 0, 60, 20),
        include_str!("../tests/snapshots/picker.txt"),
        Box::new(move |buf| {
            Picker::new(
                &layout,
                &["Graphite Dark", "Graphite Light", "Nord", "Solarized"],
                1,
                &LIGHT,
            )
            .render(buf.area, buf)
        }),
    );
    vec![tabs, rail, sidebar, pane, legend, picker]
}

fn bench(c: &mut Criterion) {
    let scenes = scenes();
    let repeats = if std::env::var("GRAPHITE_BENCH_NEGATIVE_CONTROL").is_ok_and(|v| v == "1") {
        16
    } else {
        1
    };
    eprintln!("Graphite draw repetitions per iteration: {repeats}");
    let reverse = std::env::var("GRAPHITE_BENCH_ORDER").is_ok_and(|v| v == "plain-first");
    for scene in &scenes {
        let mut group = c.benchmark_group(scene.name);
        let order = if reverse {
            ["plain", "graphite"]
        } else {
            ["graphite", "plain"]
        };
        for side in order {
            let mut buffer = Buffer::empty(scene.area);
            group.bench_function(side, |b| {
                b.iter(|| {
                    buffer.reset();
                    if side == "graphite" {
                        for _ in 0..repeats {
                            (scene.draw)(&mut buffer);
                        }
                    } else {
                        (&scene.plain).render(scene.area, &mut buffer);
                    }
                    black_box(&buffer);
                })
            });
        }
        group.finish();
    }
    let mut group = c.benchmark_group("frame");
    for side in if reverse {
        ["plain", "graphite"]
    } else {
        ["graphite", "plain"]
    } {
        let mut buffers: Vec<_> = scenes.iter().map(|s| Buffer::empty(s.area)).collect();
        group.bench_function(side, |b| {
            b.iter(|| {
                for (scene, buffer) in scenes.iter().zip(&mut buffers) {
                    buffer.reset();
                    if side == "graphite" {
                        for _ in 0..repeats {
                            (scene.draw)(buffer);
                        }
                    } else {
                        (&scene.plain).render(scene.area, buffer);
                    }
                    black_box(&*buffer);
                }
            })
        });
    }
    group.finish();
}
criterion_group! {name=benches; config=Criterion::default().sample_size(30).warm_up_time(Duration::from_millis(300)).measurement_time(Duration::from_millis(500)); targets=bench}
criterion_main!(benches);
