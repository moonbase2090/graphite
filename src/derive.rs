//! Theme inputs and Graphite token derivation.

use crate::color::BarColor;
use crate::tokens::{Rgb, ThemeVariant, Tokens};

/// The theme fields needed to derive renderer-independent Graphite tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeSource {
    /// Stable theme identifier used for the built-in brief-token check.
    pub id: String,
    /// Dark or light direction of the theme.
    pub variant: ThemeVariant,
    /// Tabs-bar source fill.
    pub chrome_bg: Rgb,
    /// Primary chrome text color.
    pub chrome_fg: Rgb,
    /// Terminal pane background.
    pub default_bg: Rgb,
    /// Terminal pane text color.
    pub default_fg: Rgb,
    /// Active tab-chip fill.
    pub tab_active_bg: Rgb,
    /// Window ground behind panes.
    pub pane_backdrop: Rgb,
    /// Pane outline color.
    pub pane_border: Rgb,
    /// Working-status badge color.
    pub active_badge: Rgb,
    /// Unseen-output badge color.
    pub unseen_badge: Rgb,
    /// Attention badge color.
    pub attention_badge: Rgb,
    /// Theme ANSI palette; index 4 supplies the blue focus tint.
    pub ansi: [Rgb; 16],
}

const BRIEF_THEMES: [&str; 3] = ["prismattyc", "prismattyc-dark", "prismattyc-light"];
const BLACK: Rgb = [0, 0, 0];
const WHITE: Rgb = [0xff, 0xff, 0xff];
const AA: f32 = 4.5;

/// Return whether a source is one of the unchanged built-in Graphite themes.
pub fn uses_brief(source: &ThemeSource) -> bool {
    let brief = crate::tokens::tokens(source.variant);
    BRIEF_THEMES.contains(&source.id.as_str())
        && source.chrome_bg == brief.bar
        && source.chrome_fg == brief.text
}

/// Derive Graphite tokens from a theme source.
pub fn derive_tokens(source: &ThemeSource) -> Tokens {
    let variant = source.variant;
    let light = variant == ThemeVariant::Light;
    let (bar, fg) = (source.chrome_bg, source.chrome_fg);
    let (pane, ink) = (source.default_bg, source.default_fg);
    let blue = source.ansi[4];
    let status_bar = if light {
        toward(bar, fg, 5)
    } else {
        toward(bar, BLACK, 38)
    };
    let mut tokens = Tokens {
        ground: source.pane_backdrop,
        bar,
        bar_line: toward(bar, fg, if light { 12 } else { 7 }),
        divider: toward(bar, fg, 12),
        tab_active: source.tab_active_bg,
        tab_active_line: light.then(|| toward(bar, fg, 30)),
        tab_hover: hover_rgb(variant, bar, fg, 0.10),
        text: fg,
        text_strong: if light { fg } else { toward(fg, WHITE, 30) },
        muted: toward(fg, bar, 30),
        tab_text: toward(fg, bar, 25),
        working: source.active_badge,
        unseen: source.unseen_badge,
        attention: source.attention_badge,
        on_attention: ink_on(source.attention_badge),
        idle: toward(fg, bar, 50),
        field: pane,
        field_line: toward(bar, fg, 16),
        key: hover_rgb(variant, bar, fg, 0.10),
        key_line: toward(bar, fg, 18),
        key_text: toward(fg, bar, 12),
        status_bar,
        status_line: toward(status_bar, fg, 7),
        chip_active: derived_tab_active_bg(variant, status_bar, fg),
        separator: toward(bar, fg, 25),
        hairline: source.pane_border,
        title_line: toward(pane, ink, 8),
        title_focus: toward(pane, blue, 10),
        title_focus_line: toward(pane, blue, 22),
        muted_focus: toward(ink, pane, 22),
        title_hover: hover_rgb(variant, pane, ink, 0.10),
        hover_outline: toward(pane, blue, 40),
        unseen_text: source.unseen_badge,
        cycle_head: crate::tokens::tokens(variant).cycle_head,
        panel: pane,
        variant,
    };
    keep_text_readable(&mut tokens);
    tokens
}

/// Return brief tokens verbatim, or derive tokens for another theme.
pub fn theme_tokens(source: &ThemeSource) -> Tokens {
    if uses_brief(source) {
        *crate::tokens::tokens(source.variant)
    } else {
        derive_tokens(source)
    }
}

/// Apply an optional bar preset to theme-derived tokens.
pub fn bar_tokens(source: &ThemeSource, bar: Option<BarColor>) -> Tokens {
    let mut tokens = theme_tokens(source);
    if let Some(bar) = bar {
        let (tabs, status) = crate::color::bar_fills(bar, source.variant);
        tokens.bar = tabs;
        tokens.status_bar = status;
        if !uses_brief(source) {
            keep_text_readable(&mut tokens);
        }
    }
    tokens
}

/// Move a color a percentage of the way toward another color.
fn toward(from: Rgb, to: Rgb, percent: u16) -> Rgb {
    crate::mix_rgb(from, to, percent.min(100) * 256 / 100)
}

fn readable(ink: Rgb, grounds: &[Rgb], target: f32) -> Rgb {
    let Some(&first) = grounds.first() else {
        return ink;
    };
    let pole = if crate::contrast_ratio(WHITE, first) >= crate::contrast_ratio(BLACK, first) {
        WHITE
    } else {
        BLACK
    };
    let worst = |color: Rgb| {
        grounds
            .iter()
            .map(|ground| crate::contrast_ratio(color, *ground))
            .fold(f32::INFINITY, f32::min)
    };
    let mut color = ink;
    for _ in 0..24 {
        if worst(color) >= target {
            break;
        }
        color = crate::mix_rgb(color, pole, 24);
    }
    color
}

fn ink_on(fill: Rgb) -> Rgb {
    if crate::contrast_ratio(WHITE, fill)
        >= crate::contrast_ratio(crate::tokens::DARK.on_attention, fill)
    {
        WHITE
    } else {
        crate::tokens::DARK.on_attention
    }
}

fn keep_text_readable(tokens: &mut Tokens) {
    let bars = [
        tokens.bar,
        tokens.status_bar,
        tokens.tab_active,
        tokens.chip_active,
    ];
    tokens.muted = readable(
        tokens.muted,
        &[tokens.bar, tokens.tab_active, tokens.field],
        AA,
    );
    tokens.tab_text = readable(tokens.tab_text, &bars, AA);
    let floor = AA
        .max(crate::contrast_ratio(tokens.muted, tokens.bar))
        .max(crate::contrast_ratio(tokens.tab_text, tokens.bar));
    tokens.text = readable(tokens.text, &bars, floor);
    tokens.text_strong = readable(tokens.text_strong, &bars, floor);
    tokens.key_text = readable(tokens.key_text, &[tokens.key], AA);
    tokens.muted_focus = readable(tokens.muted_focus, &[tokens.title_focus], AA);
    tokens.unseen_text = readable(tokens.unseen_text, &[tokens.bar, tokens.status_bar], AA);
    tokens.on_attention = readable(tokens.on_attention, &[tokens.attention], AA);
}

fn derived_tab_active_bg(variant: ThemeVariant, chrome_bg: Rgb, chrome_fg: Rgb) -> Rgb {
    const TAB_ACTIVE_DARK_BLEND: f32 = 0.08;
    const TAB_ACTIVE_LIGHT_BLEND: f32 = 0.06;
    const MIN_TAB_ACTIVE_DELTA: u8 = 8;
    const MAX_TAB_ACTIVE_DELTA: u8 = 40;
    let t = match variant {
        ThemeVariant::Dark => TAB_ACTIVE_DARK_BLEND,
        ThemeVariant::Light => TAB_ACTIVE_LIGHT_BLEND,
    };
    let mix = |from: u8, toward: u8| {
        (f32::from(from) + (f32::from(toward) - f32::from(from)) * t).round() as u8
    };
    let mut out = [
        mix(chrome_bg[0], chrome_fg[0]),
        mix(chrome_bg[1], chrome_fg[1]),
        mix(chrome_bg[2], chrome_fg[2]),
    ];
    let max_delta = (0..3)
        .map(|i| out[i].abs_diff(chrome_bg[i]))
        .max()
        .unwrap_or(0);
    if max_delta == 0 {
        return shade(chrome_bg, 8);
    }
    let scale = if max_delta < MIN_TAB_ACTIVE_DELTA {
        f32::from(MIN_TAB_ACTIVE_DELTA) / f32::from(max_delta)
    } else if max_delta > MAX_TAB_ACTIVE_DELTA {
        f32::from(MAX_TAB_ACTIVE_DELTA) / f32::from(max_delta)
    } else {
        return out;
    };
    for i in 0..3 {
        let signed = i32::from(out[i]) - i32::from(chrome_bg[i]);
        let scaled = (signed as f32 * scale).round() as i32;
        out[i] = (i32::from(chrome_bg[i]) + scaled).clamp(0, 255) as u8;
    }
    out
}

fn shade(rgb: Rgb, percent: u32) -> Rgb {
    const MIN_BACKDROP_DELTA: u8 = 8;
    let step = |channel: u8| {
        let scaled = (u32::from(channel) * percent) / 100;
        let delta = scaled.max(u32::from(MIN_BACKDROP_DELTA)) as u8;
        match channel.checked_sub(delta) {
            Some(darker) => darker,
            None => channel.saturating_add(delta),
        }
    };
    [step(rgb[0]), step(rgb[1]), step(rgb[2])]
}

fn hover_rgb(variant: ThemeVariant, base: Rgb, chrome_fg: Rgb, blend: f32) -> Rgb {
    const HOVER_LIGHT_SCALE: f32 = 0.8;
    const MAX_HOVER_DELTA: u8 = 30;
    let t = blend.clamp(0.0, 0.3)
        * if variant == ThemeVariant::Light {
            HOVER_LIGHT_SCALE
        } else {
            1.0
        };
    let mix = |from: u8, toward: u8| {
        (f32::from(from) + (f32::from(toward) - f32::from(from)) * t).round() as u8
    };
    let mut out = [
        mix(base[0], chrome_fg[0]),
        mix(base[1], chrome_fg[1]),
        mix(base[2], chrome_fg[2]),
    ];
    let max_delta = (0..3).map(|i| out[i].abs_diff(base[i])).max().unwrap_or(0);
    if max_delta > MAX_HOVER_DELTA {
        let scale = f32::from(MAX_HOVER_DELTA) / f32::from(max_delta);
        for i in 0..3 {
            out[i] = (f32::from(base[i]) + (f32::from(out[i]) - f32::from(base[i])) * scale).round()
                as u8;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{bar_tokens, derive_tokens, theme_tokens, uses_brief, ThemeSource};
    use crate::{bar_fills, contrast_ratio, BarColor, ThemeVariant, DARK, LIGHT};

    const fn rgb(hex: u32) -> [u8; 3] {
        [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
    }

    #[allow(clippy::too_many_arguments)]
    fn source(
        id: &str,
        variant: ThemeVariant,
        chrome_bg: u32,
        chrome_fg: u32,
        default_bg: u32,
        default_fg: u32,
        tab_active_bg: u32,
        pane_backdrop: u32,
        pane_border: u32,
        active_badge: u32,
        unseen_badge: u32,
        attention_badge: u32,
        ansi_blue: u32,
    ) -> ThemeSource {
        let mut ansi = [[0, 0, 0]; 16];
        ansi[4] = rgb(ansi_blue);
        ThemeSource {
            id: id.to_owned(),
            variant,
            chrome_bg: rgb(chrome_bg),
            chrome_fg: rgb(chrome_fg),
            default_bg: rgb(default_bg),
            default_fg: rgb(default_fg),
            tab_active_bg: rgb(tab_active_bg),
            pane_backdrop: rgb(pane_backdrop),
            pane_border: rgb(pane_border),
            active_badge: rgb(active_badge),
            unseen_badge: rgb(unseen_badge),
            attention_badge: rgb(attention_badge),
            ansi,
        }
    }

    fn japanesque() -> ThemeSource {
        source(
            "japanesque",
            ThemeVariant::Dark,
            0x181818,
            0xf7f6ec,
            0x1e1e1e,
            0xf7f6ec,
            0x2a2a29,
            0x161616,
            0x595b59,
            0x7bb75b,
            0xe9b32a,
            0xff6b6b,
            0x4c9ad4,
        )
    }

    fn expected_japanesque() -> crate::Tokens {
        crate::Tokens {
            ground: rgb(0x161616),
            bar: rgb(0x181818),
            bar_line: rgb(0x262626),
            divider: rgb(0x323230),
            tab_active: rgb(0x2a2a29),
            tab_active_line: None,
            tab_hover: rgb(0x2e2e2d),
            text: rgb(0xf7f6ec),
            text_strong: rgb(0xf9f8f1),
            muted: rgb(0xb4b4ad),
            tab_text: rgb(0xbfbeb7),
            working: rgb(0x7bb75b),
            unseen: rgb(0xe9b32a),
            attention: rgb(0xff6b6b),
            on_attention: rgb(0x1a0d0b),
            idle: rgb(0x878782),
            field: rgb(0x1e1e1e),
            field_line: rgb(0x3a3a39),
            key: rgb(0x2e2e2d),
            key_line: rgb(0x403f3e),
            key_text: rgb(0xdcdbd3),
            status_bar: rgb(0x0e0e0e),
            status_line: rgb(0x1d1d1c),
            chip_active: rgb(0x212120),
            separator: rgb(0x4f4f4d),
            hairline: rgb(0x595b59),
            title_line: rgb(0x2e2e2e),
            title_focus: rgb(0x222a2f),
            title_focus_line: rgb(0x283945),
            muted_focus: rgb(0xc7c6be),
            title_hover: rgb(0x343433),
            hover_outline: rgb(0x304f66),
            unseen_text: rgb(0xe9b32a),
            cycle_head: rgb(0xd6e8ff),
            panel: rgb(0x1e1e1e),
            variant: ThemeVariant::Dark,
        }
    }

    #[test]
    fn brief_ids_keep_static_tokens_verbatim() {
        for id in ["prismattyc", "prismattyc-dark", "prismattyc-light"] {
            let dark = source(
                id,
                ThemeVariant::Dark,
                0x15181d,
                0xe6e9ee,
                0x181b21,
                0xe6e9ee,
                0x252a33,
                0x101216,
                0x262a32,
                0x4cc98a,
                0xf2b84b,
                0xff7a6b,
                0x5aa2ff,
            );
            assert!(uses_brief(&dark));
            assert_eq!(theme_tokens(&dark), DARK);

            let light = source(
                id,
                ThemeVariant::Light,
                0xeef0f3,
                0x1f2329,
                0xffffff,
                0x1f2329,
                0xffffff,
                0xe9ecf0,
                0xd5d9df,
                0x1f8a55,
                0xb87a0a,
                0xc2392b,
                0x2f6fd0,
            );
            assert!(uses_brief(&light));
            assert_eq!(theme_tokens(&light), LIGHT);
        }
    }

    #[test]
    fn non_brief_tokens_pin_every_derived_field() {
        let theme = japanesque();
        assert!(!uses_brief(&theme));
        assert_eq!(derive_tokens(&theme), expected_japanesque());
    }

    #[test]
    fn explicit_bar_preset_replaces_both_fills_only() {
        let theme = japanesque();
        let own = bar_tokens(&theme, None);
        assert_eq!(own, expected_japanesque());
        let harbor = bar_tokens(&theme, Some(BarColor::Harbor));
        let (bar, status_bar) = bar_fills(BarColor::Harbor, ThemeVariant::Dark);
        assert_eq!(harbor.bar, bar);
        assert_eq!(harbor.status_bar, status_bar);
        let mut expected = own;
        expected.bar = bar;
        expected.status_bar = status_bar;
        assert_eq!(harbor, expected);
    }

    #[test]
    fn chrome_overrides_make_brief_ids_derive() {
        let mut theme = source(
            "prismattyc-dark",
            ThemeVariant::Dark,
            0x203040,
            0xf0e0d0,
            0x181b21,
            0xe6e9ee,
            0x252a33,
            0x101216,
            0x262a32,
            0x4cc98a,
            0xf2b84b,
            0xff7a6b,
            0x5aa2ff,
        );
        assert!(!uses_brief(&theme));
        let derived = theme_tokens(&theme);
        assert_eq!(derived.bar, rgb(0x203040));
        assert_eq!(derived.text, rgb(0xf0e0d0));

        theme.id = "japanesque".to_owned();
        theme.chrome_bg = rgb(0x203040);
        theme.chrome_fg = rgb(0xf0e0d0);
        assert!(!uses_brief(&theme));
        let derived = theme_tokens(&theme);
        assert_eq!(derived.bar, rgb(0x203040));
        assert_eq!(derived.text, rgb(0xf0e0d0));
    }

    #[test]
    fn derived_text_stays_readable() {
        let theme = japanesque();
        let tokens = derive_tokens(&theme);
        for (foreground, background) in [
            (tokens.text, tokens.bar),
            (tokens.tab_text, tokens.bar),
            (tokens.muted, tokens.bar),
            (tokens.text_strong, tokens.tab_active),
            (tokens.muted, tokens.tab_active),
            (tokens.on_attention, tokens.attention),
            (tokens.muted, tokens.field),
            (tokens.key_text, tokens.key),
        ] {
            assert!(contrast_ratio(foreground, background) >= 4.5);
        }
        assert!(
            contrast_ratio(tokens.text, tokens.bar) >= contrast_ratio(tokens.muted, tokens.bar)
        );

        let murky = source(
            "japanesque",
            ThemeVariant::Dark,
            0x404040,
            0x505050,
            0x1e1e1e,
            0xf7f6ec,
            0x2a2a29,
            0x161616,
            0x595b59,
            0x7bb75b,
            0xe9b32a,
            0xff6b6b,
            0x4c9ad4,
        );
        let tokens = derive_tokens(&murky);
        assert!(contrast_ratio(tokens.text, tokens.bar) >= 4.5);
        assert!(contrast_ratio(tokens.tab_text, tokens.bar) >= 4.5);
    }
}
