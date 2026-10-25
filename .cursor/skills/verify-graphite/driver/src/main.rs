use graphite_core::{
    bar_hit, bar_layout, contrast_ratio, ellipsize, legend_keys, status_width, tokens, BarHit, Dot,
    Face, LegendRun, PaneStatus, Rect, TabText, TextMetrics, ThemeVariant,
};

struct CharCount;

impl TextMetrics for CharCount {
    fn width(&self, _face: Face, _px: f32, text: &str) -> f32 {
        text.chars().count() as f32
    }
}

struct CharTimesPx;

impl TextMetrics for CharTimesPx {
    fn width(&self, _face: Face, px: f32, text: &str) -> f32 {
        text.chars().count() as f32 * px
    }
}

fn main() {
    let feature = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!(
            "usage: verify-graphite <ellipsize|theme-tokens|pane-status|legend-keys|tabs-bar>"
        );
        std::process::exit(2);
    });
    match feature.as_str() {
        "ellipsize" => drive_ellipsize(),
        "theme-tokens" => drive_theme_tokens(),
        "pane-status" => drive_pane_status(),
        "legend-keys" => drive_legend_keys(),
        "tabs-bar" => drive_tabs_bar(),
        other => {
            eprintln!("unknown feature {other}");
            std::process::exit(2);
        }
    }
}

fn drive_ellipsize() {
    let metrics = CharCount;
    let fit_input = "short";
    let fit_budget = 8.0;
    let cut_input = "abcdef";
    let cut_budget = 4.0;
    let unchanged = ellipsize(&metrics, Face::Regular, 12.0, fit_input, fit_budget);
    let cut = ellipsize(&metrics, Face::Regular, 12.0, cut_input, cut_budget);
    println!("feature ellipsize");
    println!("unchanged input={fit_input} budget={fit_budget:.0} result={unchanged}");
    println!("cut input={cut_input} budget={cut_budget:.0} result={cut}");
    assert_eq!(unchanged, "short");
    assert_eq!(cut, "abc…");
    for (kind, input, budget, expected) in [
        ("flag-narrow", "🇺🇸abcd", 2.0, "…"),
        ("flag-fit", "🇺🇸abcd", 3.0, "🇺🇸…"),
        ("family-narrow", "👨‍👩‍👧‍👦abcd", 2.0, "…"),
        ("family-fit", "👨‍👩‍👧‍👦abcd", 8.0, "👨‍👩‍👧‍👦…"),
        ("accent-narrow", "a\u{301}bc", 2.0, "…"),
        ("accent-fit", "a\u{301}bc", 3.0, "a\u{301}…"),
        ("no-space", "🇺🇸abcd", 0.0, ""),
    ] {
        let result = ellipsize(&metrics, Face::Regular, 12.0, input, budget);
        println!("grapheme kind={kind} input={input} budget={budget:.0} result={result}");
        assert_eq!(result, expected, "{kind}");
    }
}

fn drive_theme_tokens() {
    let dark = tokens(ThemeVariant::Dark);
    let light = tokens(ThemeVariant::Light);
    let black_white = contrast_ratio([0, 0, 0], [255, 255, 255]);
    let text_ground = contrast_ratio(dark.text, dark.ground);
    println!(
        "dark-ground {} {} {}",
        dark.ground[0], dark.ground[1], dark.ground[2]
    );
    println!("dark-variant {:?}", dark.variant);
    println!(
        "light-text {} {} {}",
        light.text[0], light.text[1], light.text[2]
    );
    println!("light-variant {:?}", light.variant);
    println!("contrast-black-white {black_white:.4}");
    println!("contrast-text-ground {text_ground:.4}");
    assert_eq!(dark.ground, [0x10, 0x12, 0x16]);
    assert_eq!(dark.variant, ThemeVariant::Dark);
    assert_eq!(light.text, [0x1f, 0x23, 0x29]);
    assert_eq!(light.variant, ThemeVariant::Light);
    assert!((black_white - 21.0).abs() < 0.0001);
    assert!(text_ground >= 4.5);
}

fn drive_pane_status() {
    let metrics = CharTimesPx;
    let attention = PaneStatus::decide(true, 4, true, true, true);
    let mail = PaneStatus::decide(false, 4, true, true, true);
    let quiet = PaneStatus::decide(false, 0, false, false, false);
    let attention_width = status_width(&metrics, 1000, attention);
    let quiet_width = status_width(&metrics, 1000, quiet);
    println!("decide-attention {attention:?}");
    println!("decide-mail {mail:?}");
    println!("attention-width {attention_width}");
    println!("quiet-width {quiet_width}");
    assert_eq!(attention, PaneStatus::Attention);
    assert_eq!(mail, PaneStatus::Mail(4));
    assert_eq!(quiet, PaneStatus::Quiet);
    assert_eq!(attention_width, 108.0);
    assert_eq!(quiet_width, 0.0);
}

fn drive_legend_keys() {
    let metrics = CharCount;
    let fitted = legend_keys(
        &metrics,
        1000,
        &[LegendRun::Text("Prismattyc [1]".to_owned())],
        80,
        48,
        1,
    );
    let dropped = legend_keys(
        &metrics,
        1000,
        &[LegendRun::Text("Prismattyc [1]".to_owned())],
        20,
        48,
        1,
    );
    println!(
        "fitted {} {} {} {} {}",
        fitted.texts.len(),
        fitted.texts[0].rect.x,
        fitted.texts[0].rect.y,
        fitted.texts[0].rect.w,
        fitted.texts[0].rect.h
    );
    println!("fitted-text {}", fitted.texts[0].text);
    println!("dropped {}", dropped.texts.len());
    assert_eq!(fitted.texts.len(), 1);
    assert_eq!(fitted.texts[0].rect, Rect::new(12, 26, 14, 22));
    assert_eq!(fitted.texts[0].text, "Prismattyc [1]");
    assert!(dropped.texts.is_empty());
}

fn drive_tabs_bar() {
    let metrics = CharTimesPx;
    let tabs = [tab("grid", true), tab("review", false), tab("notes", false)];
    let layout = bar_layout(
        &metrics,
        1000,
        1440,
        0,
        "release-lab",
        &tabs,
        "Ctrl Shift P",
    );
    let dropdown = bar_hit(&layout, 10, 7, false);
    let first = bar_hit(&layout, 226, 7, false);
    println!(
        "bar {} {} {} {}",
        layout.bar.x, layout.bar.y, layout.bar.w, layout.bar.h
    );
    println!("space-label {}", layout.space_label);
    println!("tabs {}", layout.tabs.len());
    println!(
        "first-chip {} {} {} {}",
        layout.tabs[0].chip.x, layout.tabs[0].chip.y, layout.tabs[0].chip.w, layout.tabs[0].chip.h
    );
    println!("hit-dropdown {}", hit_name(dropdown));
    println!("hit-first {}", hit_name(first));
    assert_eq!(layout.bar, Rect::new(0, 0, 1440, 44));
    assert_eq!(layout.space_label, "release-lab");
    assert_eq!(layout.tabs.len(), 3);
    assert_eq!(layout.tabs[0].chip, Rect::new(226, 7, 107, 30));
    assert_eq!(dropdown, Some(BarHit::SpaceMenu));
    assert_eq!(
        first,
        Some(BarHit::Tab {
            index: 0,
            close: false
        })
    );
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

fn hit_name(hit: Option<BarHit>) -> String {
    match hit {
        Some(BarHit::SpaceMenu) => "space-menu".to_owned(),
        Some(BarHit::Tab {
            index,
            close: false,
        }) => format!("tab {index}"),
        Some(BarHit::Tab { index, close: true }) => format!("tab {index} close"),
        Some(BarHit::NewTab) => "new-tab".to_owned(),
        Some(BarHit::Command) => "command".to_owned(),
        Some(BarHit::EmptyEnd) => "empty-end".to_owned(),
        None => "none".to_owned(),
    }
}
