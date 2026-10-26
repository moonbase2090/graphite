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

/// Every color field of [`Tokens`], in the order their sentinels are assigned.
macro_rules! color_fields {
    ($m:ident) => {
        $m!(
            ground,
            bar,
            bar_line,
            divider,
            tab_active,
            tab_hover,
            text,
            text_strong,
            muted,
            tab_text,
            working,
            unseen,
            attention,
            on_attention,
            idle,
            field,
            field_line,
            key,
            key_line,
            key_text,
            status_bar,
            status_line,
            chip_active,
            separator,
            hairline,
            title_line,
            title_focus,
            title_focus_line,
            muted_focus,
            title_hover,
            hover_outline,
            unseen_text,
            cycle_head,
            panel
        )
    };
}

fn role_of(palette: &[TerminalToken], name: &str) -> Ansi16 {
    palette
        .iter()
        .find(|entry| entry.name == name)
        .map_or(Ansi16::White, |entry| entry.ansi16)
}

/// Returns `t` with each color field replaced by a unique sentinel, and each field's ANSI-16 role.
///
/// A ratatui buffer keeps only RGB, so tokens that share a value (`chip_active`
/// and `tab_hover` in the dark theme) would collapse into one ANSI slot. The
/// sentinel `[0xFE, 0, i]` names field `i`, and [`sgr`] maps it back to that
/// field's own role.
fn sentinel(t: &Tokens) -> (Tokens, Vec<Ansi16>) {
    let palette = terminal_palette(t);
    let mut roles = Vec::new();
    let mut out = *t;
    macro_rules! mark {
        ($($f:ident),*) => {$(
            out.$f = [0xFE, 0, roles.len() as u8];
            roles.push(role_of(&palette, stringify!($f)));
        )*};
    }
    color_fields!(mark);
    if t.tab_active_line.is_some() {
        out.tab_active_line = Some([0xFE, 0, roles.len() as u8]);
        roles.push(role_of(&palette, "tab_active_line"));
    }
    (out, roles)
}

/// Maps a drawn color to the SGR parameters for the chosen depth.
fn sgr(c: Color, depth: Depth, fg: bool, roles: &[Ansi16]) -> Option<String> {
    let base = if fg { 38 } else { 48 };
    let Color::Rgb(r, g, b) = c else {
        // `Reset` means the terminal default. Emit it so an unstyled cell does not inherit the previous color.
        return (c == Color::Reset).then(|| format!("{}", if fg { 39 } else { 49 }));
    };
    Some(match depth {
        Depth::TrueColor => format!("{base};2;{r};{g};{b}"),
        Depth::Ansi256 => format!("{base};5;{}", xterm256([r, g, b])),
        Depth::Ansi16 => {
            let slot = if r == 0xFE && g == 0 {
                roles.get(usize::from(b)).copied().unwrap_or(Ansi16::White) as u8
            } else {
                Ansi16::White as u8
            };
            let code = if slot < 8 { 30 + slot } else { 90 + slot - 8 };
            format!("{}", if fg { code } else { code + 10 })
        }
    })
}

fn dump(buf: &Buffer, depth: Depth, roles: &[Ansi16]) {
    let a = buf.area;
    for y in a.y..a.bottom() {
        let mut line = String::new();
        for x in a.x..a.right() {
            let c = &buf[(x, y)];
            for (color, fg) in [(c.fg, true), (c.bg, false)] {
                if let Some(code) = sgr(color, depth, fg, roles) {
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
    let (drawn, roles) = sentinel(t);
    let t = if depth == Depth::Ansi16 { &drawn } else { t };
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
    dump(&buf, depth, &roles);

    let area = Rect::new(0, 0, cols, 2);
    let mut buf = Buffer::empty(area);
    SideRail::new(&["main", "docs"], 0, t).render(area, &mut buf);
    dump(&buf, depth, &roles);

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
    dump(&buf, depth, &roles);

    let area = Rect::new(0, 0, 40, 6);
    let slot = PxRect::new(0, 0, 40 * CELL_W, 6 * CELL_H);
    let mut buf = Buffer::empty(area);
    PaneChrome::new(slot, "agent-1", PaneStatus::Mail(2), true, t).render(area, &mut buf);
    dump(&buf, depth, &roles);

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
    dump(&buf, depth, &roles);

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
    dump(&buf, depth, &roles);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(c: [u8; 3]) -> Color {
        Color::Rgb(c[0], c[1], c[2])
    }

    #[test]
    fn tokens_with_equal_values_keep_their_own_ansi_roles() {
        let (drawn, roles) = sentinel(&DARK);
        assert_eq!(DARK.chip_active, DARK.tab_hover);
        assert_eq!(
            sgr(rgb(drawn.chip_active), Depth::Ansi16, false, &roles).unwrap(),
            "40"
        );
        assert_eq!(
            sgr(rgb(drawn.tab_hover), Depth::Ansi16, false, &roles).unwrap(),
            "100"
        );
    }

    #[test]
    fn reset_emits_the_default_colors_so_cells_do_not_inherit() {
        let (_, roles) = sentinel(&DARK);
        assert_eq!(
            sgr(Color::Reset, Depth::TrueColor, false, &roles).unwrap(),
            "49"
        );
        assert_eq!(
            sgr(Color::Reset, Depth::Ansi16, true, &roles).unwrap(),
            "39"
        );
    }

    #[test]
    fn truecolor_and_256_still_use_the_real_values() {
        let (_, roles) = sentinel(&DARK);
        assert_eq!(
            sgr(Color::Rgb(31, 35, 42), Depth::TrueColor, false, &roles).unwrap(),
            "48;2;31;35;42"
        );
    }
}
