//! Pure color math and bar presets for Graphite.

use crate::tokens::{Rgb, ThemeVariant, Tokens};

/// A background preset for the tabs and spaces bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarColor {
    Graphite,
    Harbor,
    Moss,
    Plum,
}

pub fn mix_rgb(_background: Rgb, _foreground: Rgb, _foreground_weight: u16) -> Rgb {
    [0, 0, 0]
}

pub fn relative_luminance(_rgb: Rgb) -> f32 {
    0.0
}

pub fn contrast_ratio(_a: Rgb, _b: Rgb) -> f32 {
    1.0
}

pub fn bar_fills(_bar: BarColor, _variant: ThemeVariant) -> (Rgb, Rgb) {
    ([0, 0, 0], [0, 0, 0])
}

pub fn bar_color_name(_bar: Option<BarColor>) -> &'static str {
    "Theme"
}

pub fn step_bar_color(
    _bar: Option<BarColor>,
    _forward: bool,
    _theme_bars: bool,
) -> Option<BarColor> {
    None
}

pub fn accent(_tokens: &Tokens, _focus: Rgb) -> Rgb {
    [0, 0, 0]
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
                assert!(
                    contrast_ratio(
                        accent(
                            if variant == ThemeVariant::Dark {
                                &DARK
                            } else {
                                &LIGHT
                            },
                            blue
                        ),
                        tabs
                    ) >= 3.0
                );
            }
        }
    }
}
