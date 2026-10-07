//! Pure color math and bar presets for Graphite.

use crate::tokens::{Rgb, ThemeVariant, Tokens};

/// A background preset for the tabs and spaces bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarColor {
    #[default]
    Graphite,
    Harbor,
    Moss,
    Plum,
}

pub fn mix_rgb(background: Rgb, foreground: Rgb, foreground_weight: u16) -> Rgb {
    let background_weight = 256u16.saturating_sub(foreground_weight);
    [
        ((u16::from(background[0]) * background_weight
            + u16::from(foreground[0]) * foreground_weight)
            / 256) as u8,
        ((u16::from(background[1]) * background_weight
            + u16::from(foreground[1]) * foreground_weight)
            / 256) as u8,
        ((u16::from(background[2]) * background_weight
            + u16::from(foreground[2]) * foreground_weight)
            / 256) as u8,
    ]
}

/// WCAG relative luminance of an sRGB color.
pub fn relative_luminance(rgb: Rgb) -> f32 {
    let linear = |channel: u8| {
        let channel = f32::from(channel) / 255.0;
        if channel <= 0.039_28 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(rgb[0]) + 0.7152 * linear(rgb[1]) + 0.0722 * linear(rgb[2])
}

/// WCAG contrast ratio between two colors (1.0 through 21.0).
pub fn contrast_ratio(a: Rgb, b: Rgb) -> f32 {
    let (a, b) = (relative_luminance(a), relative_luminance(b));
    let (high, low) = if a > b { (a, b) } else { (b, a) };
    (high + 0.05) / (low + 0.05)
}

/// Tabs-bar and spaces-bar fills for a preset and theme variant.
pub fn bar_fills(bar: BarColor, variant: ThemeVariant) -> (Rgb, Rgb) {
    use BarColor::{Graphite, Harbor, Moss, Plum};
    use ThemeVariant::{Dark, Light};

    match (bar, variant) {
        (Graphite, Dark) => ([0x15, 0x18, 0x1d], [0x0d, 0x0f, 0x12]),
        (Graphite, Light) => ([0xee, 0xf0, 0xf3], [0xe4, 0xe7, 0xec]),
        (Harbor, Dark) => ([0x15, 0x21, 0x31], [0x0e, 0x17, 0x22]),
        (Harbor, Light) => ([0xe3, 0xed, 0xf8], [0xd6, 0xe3, 0xf2]),
        (Moss, Dark) => ([0x17, 0x22, 0x1b], [0x0f, 0x17, 0x12]),
        (Moss, Light) => ([0xe4, 0xf0, 0xe7], [0xd7, 0xe7, 0xdb]),
        (Plum, Dark) => ([0x21, 0x1a, 0x27], [0x17, 0x12, 0x1c]),
        (Plum, Light) => ([0xf4, 0xec, 0xe0], [0xeb, 0xe0, 0xcf]),
    }
}

/// Display name for a preset; `None` means the theme's bars.
pub fn bar_color_name(bar: Option<BarColor>) -> &'static str {
    match bar {
        None => "Theme",
        Some(BarColor::Graphite) => "Graphite",
        Some(BarColor::Harbor) => "Harbor",
        Some(BarColor::Moss) => "Moss",
        Some(BarColor::Plum) => "Plum",
    }
}

/// Step through the bar-color cycle, wrapping at either end.
pub fn step_bar_color(bar: Option<BarColor>, forward: bool, theme_bars: bool) -> Option<BarColor> {
    const PRESETS: [Option<BarColor>; 5] = [
        None,
        Some(BarColor::Graphite),
        Some(BarColor::Harbor),
        Some(BarColor::Moss),
        Some(BarColor::Plum),
    ];
    let order = if theme_bars {
        &PRESETS[..]
    } else {
        &PRESETS[1..]
    };
    let current = bar.or(if theme_bars {
        None
    } else {
        Some(BarColor::Graphite)
    });
    let index = order
        .iter()
        .position(|preset| *preset == current)
        .unwrap_or(0);
    let next = if forward {
        index.saturating_add(1) % order.len()
    } else {
        index.saturating_add(order.len().saturating_sub(1)) % order.len()
    };
    order[next]
}

/// Nudge a focus color until it has the 3:1 non-text contrast required by WCAG.
pub fn accent(tokens: &Tokens, focus: Rgb) -> Rgb {
    let mut color = focus;
    let light_bar = relative_luminance(tokens.bar) > relative_luminance(tokens.text);
    for _ in 0..24 {
        if contrast_ratio(color, tokens.bar) >= 3.0 {
            break;
        }
        color = if light_bar {
            mix_rgb(color, [0, 0, 0], 24)
        } else {
            mix_rgb(color, [255, 255, 255], 24)
        };
    }
    color
}

#[cfg(test)]
mod tests {
    use super::{
        accent, bar_color_name, bar_fills, contrast_ratio, relative_luminance, step_bar_color,
        BarColor,
    };
    use crate::tokens::{ThemeVariant, DARK, LIGHT};

    const fn rgb(hex: u32) -> [u8; 3] {
        [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
    }

    #[test]
    fn wcag_reference_points_are_pinned() {
        assert!((relative_luminance(rgb(0x000000)) - 0.0).abs() < f32::EPSILON);
        assert!((relative_luminance(rgb(0xffffff)) - 1.0).abs() < f32::EPSILON);
        assert!((contrast_ratio(rgb(0x000000), rgb(0xffffff)) - 21.0).abs() < 0.0001);
        assert!((contrast_ratio(rgb(0x777777), rgb(0xffffff)) - 4.4780893).abs() < 0.0001);
    }

    #[test]
    fn bar_preset_table_is_pinned_for_both_variants() {
        assert_eq!(
            bar_fills(BarColor::Graphite, ThemeVariant::Dark),
            (rgb(0x15181d), rgb(0x0d0f12))
        );
        assert_eq!(
            bar_fills(BarColor::Graphite, ThemeVariant::Light),
            (rgb(0xeef0f3), rgb(0xe4e7ec))
        );
        assert_eq!(
            bar_fills(BarColor::Harbor, ThemeVariant::Dark),
            (rgb(0x152131), rgb(0x0e1722))
        );
        assert_eq!(
            bar_fills(BarColor::Harbor, ThemeVariant::Light),
            (rgb(0xe3edf8), rgb(0xd6e3f2))
        );
        assert_eq!(
            bar_fills(BarColor::Moss, ThemeVariant::Dark),
            (rgb(0x17221b), rgb(0x0f1712))
        );
        assert_eq!(
            bar_fills(BarColor::Moss, ThemeVariant::Light),
            (rgb(0xe4f0e7), rgb(0xd7e7db))
        );
        assert_eq!(
            bar_fills(BarColor::Plum, ThemeVariant::Dark),
            (rgb(0x211a27), rgb(0x17121c))
        );
        assert_eq!(
            bar_fills(BarColor::Plum, ThemeVariant::Light),
            (rgb(0xf4ece0), rgb(0xebe0cf))
        );
    }

    #[test]
    fn bar_cycle_wraps_and_theme_bars_include_none() {
        let presets = [
            Some(BarColor::Graphite),
            Some(BarColor::Harbor),
            Some(BarColor::Moss),
            Some(BarColor::Plum),
        ];
        for (index, preset) in presets.iter().enumerate() {
            assert_eq!(
                step_bar_color(*preset, true, false),
                presets[(index + 1) % 4]
            );
            assert_eq!(
                step_bar_color(*preset, false, false),
                presets[(index + 3) % 4]
            );
        }
        assert_eq!(step_bar_color(None, true, false), Some(BarColor::Harbor));

        let with_theme = [None, presets[0], presets[1], presets[2], presets[3]];
        for (index, preset) in with_theme.iter().enumerate() {
            assert_eq!(
                step_bar_color(*preset, true, true),
                with_theme[(index + 1) % 5]
            );
            assert_eq!(
                step_bar_color(*preset, false, true),
                with_theme[(index + 4) % 5]
            );
        }
        assert_eq!(bar_color_name(Some(BarColor::Plum)), "Plum");
        assert_eq!(bar_color_name(None), "Theme");
    }

    #[test]
    fn accent_reaches_three_to_one_on_each_bar() {
        for focus in [rgb(0x62a8ff), rgb(0xffd866), rgb(0x4cd18b), rgb(0xff6b6b)] {
            for tokens in [&DARK, &LIGHT] {
                assert!(contrast_ratio(accent(tokens, focus), tokens.bar) >= 3.0);
            }
        }
        assert_eq!(accent(&DARK, rgb(0x62a8ff)), rgb(0x62a8ff));
    }

    #[test]
    fn token_text_pairs_meet_wcag_aa() {
        for tokens in [&DARK, &LIGHT] {
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
        }
    }

    #[test]
    fn preset_text_pairs_meet_brief_contrast() {
        let blue = rgb(0x62a8ff);
        for variant in [ThemeVariant::Dark, ThemeVariant::Light] {
            for bar in [
                BarColor::Graphite,
                BarColor::Harbor,
                BarColor::Moss,
                BarColor::Plum,
            ] {
                let (tabs, status) = bar_fills(bar, variant);
                let text = match variant {
                    ThemeVariant::Dark => rgb(0xaeb5c1),
                    ThemeVariant::Light => rgb(0x4a525e),
                };
                let strong = match variant {
                    ThemeVariant::Dark => rgb(0xe6e9ee),
                    ThemeVariant::Light => rgb(0x1f2329),
                };
                assert!((contrast_ratio(text, tabs) * 10.0).round() >= 67.0);
                assert!(contrast_ratio(strong, status) >= 4.6);
                let mut themed = if variant == ThemeVariant::Dark {
                    DARK
                } else {
                    LIGHT
                };
                themed.bar = tabs;
                assert!(contrast_ratio(accent(&themed, blue), tabs) >= 3.0);
            }
        }
    }
}
