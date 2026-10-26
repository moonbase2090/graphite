//! Startup-only terminal fallback resolution.

use crate::color_space::oklab;
use crate::{Rgb, ThemeVariant, Tokens};

/// Semantic ANSI palette slot. The terminal owns the RGB values for these slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ansi16 {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}

/// One precomputed token. Resolve once when the theme changes, outside rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalToken {
    pub name: &'static str,
    pub rgb: Rgb,
    pub xterm256: u8,
    pub ansi16: Ansi16,
}

/// Resolve every present color token in declaration order.
///
/// ANSI slots are semantic, independent of theme RGB values. Optional outlines
/// are omitted when absent. The returned table allocates once at startup.
#[must_use]
pub fn terminal_palette(tokens: &Tokens) -> Vec<TerminalToken> {
    use Ansi16::*;
    let light = tokens.variant == ThemeVariant::Light;
    let background = if light { White } else { Black };
    let foreground = if light { Black } else { White };
    let strong = if light { Black } else { BrightWhite };
    let surface = if light { BrightWhite } else { Black };
    let hover = if light { White } else { BrightBlack };
    let mut entries = Vec::with_capacity(41);
    macro_rules! add {
        ($field:ident, $ansi:expr) => {
            entries.push(TerminalToken {
                name: stringify!($field),
                rgb: tokens.$field,
                xterm256: xterm256(tokens.$field),
                ansi16: $ansi,
            });
        };
    }
    add!(ground, background);
    add!(bar, background);
    add!(bar_line, BrightBlack);
    add!(divider, BrightBlack);
    add!(tab_active, surface);
    if let Some(rgb) = tokens.tab_active_line {
        entries.push(TerminalToken {
            name: "tab_active_line",
            rgb,
            xterm256: xterm256(rgb),
            ansi16: BrightBlack,
        });
    }
    add!(tab_hover, hover);
    add!(text, foreground);
    add!(text_strong, strong);
    add!(muted, BrightBlack);
    add!(tab_text, foreground);
    add!(working, Green);
    add!(unseen, Yellow);
    add!(attention, Red);
    add!(on_attention, if light { BrightWhite } else { Black });
    add!(idle, BrightBlack);
    add!(field, surface);
    add!(field_line, BrightBlack);
    add!(key, hover);
    add!(key_line, BrightBlack);
    add!(key_text, foreground);
    add!(status_bar, background);
    add!(status_line, BrightBlack);
    add!(chip_active, surface);
    add!(separator, BrightBlack);
    add!(hairline, BrightBlack);
    add!(title_line, BrightBlack);
    add!(title_focus, Blue);
    add!(title_focus_line, Blue);
    add!(muted_focus, BrightBlack);
    add!(title_hover, hover);
    add!(hover_outline, Blue);
    add!(unseen_text, Yellow);
    add!(cycle_head, if light { Blue } else { BrightWhite });
    add!(panel, surface);
    #[cfg(feature = "series")]
    {
        const SERIES: [(&str, Ansi16); 6] = [
            ("series_1", Blue),
            ("series_2", Green),
            ("series_3", Yellow),
            ("series_4", Magenta),
            ("series_5", Red),
            ("series_6", Cyan),
        ];
        for ((name, ansi16), rgb) in SERIES.into_iter().zip(tokens.series) {
            entries.push(TerminalToken {
                name,
                rgb,
                xterm256: xterm256(rgb),
                ansi16,
            });
        }
    }
    entries
}

/// Nearest fixed xterm color by squared OKLab distance, ties to the lower index.
///
/// Searches indices 16 through 255. Slots 0 through 15 belong to the host theme
/// and have no portable RGB values. This function is intended for theme setup.
#[must_use]
pub fn xterm256(rgb: Rgb) -> u8 {
    let target = oklab(rgb);
    let mut best = 16;
    let mut distance = f64::INFINITY;
    for index in 16..=255 {
        let candidate = oklab(xterm_rgb(index));
        let next = target
            .iter()
            .zip(candidate)
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        if next < distance {
            distance = next;
            best = index;
        }
    }
    best
}

fn xterm_rgb(index: u8) -> Rgb {
    if index >= 232 {
        return [8 + (index - 232) * 10; 3];
    }
    let index = index - 16;
    let levels = [0, 95, 135, 175, 215, 255];
    [
        levels[(index / 36) as usize],
        levels[(index / 6 % 6) as usize],
        levels[(index % 6) as usize],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DARK, LIGHT};

    #[test]
    fn built_in_tables_match_reviewed_reference() {
        use std::fmt::Write;
        let mut table =
            String::from("| Theme | Token | RGB | xterm-256 | ANSI-16 |\n|---|---|---|---|---|\n");
        for (theme, tokens) in [("Dark", DARK), ("Light", LIGHT)] {
            for entry in terminal_palette(&tokens)
                .into_iter()
                .filter(|entry| !entry.name.starts_with("series_"))
            {
                let [r, g, b] = entry.rgb;
                writeln!(
                    table,
                    "| {theme} | {} | #{r:02X}{g:02X}{b:02X} | {} | {:?} |",
                    entry.name, entry.xterm256, entry.ansi16
                )
                .unwrap();
            }
        }
        assert_eq!(table, include_str!("../docs/terminal-palettes.md"));
    }

    #[test]
    fn fixed_palette_endpoints_and_cube_steps() {
        assert_eq!(xterm256([0, 0, 0]), 16);
        assert_eq!(xterm256([255, 255, 255]), 231);
        assert_eq!(xterm256([255, 0, 0]), 196);
        assert_eq!(xterm256([0, 255, 0]), 46);
        assert_eq!(xterm256([0, 0, 255]), 21);
        assert_eq!(xterm256([95, 135, 175]), 67);
        assert_eq!(xterm256([8, 8, 8]), 232);
        assert_eq!(xterm256([238, 238, 238]), 255);
    }

    #[test]
    fn host_palette_semantics_ignore_rgb_overrides() {
        let mut tokens = DARK;
        tokens.attention = [0, 255, 0];
        let palette = terminal_palette(&tokens);
        let attention = palette
            .iter()
            .find(|entry| entry.name == "attention")
            .unwrap();
        assert_eq!(attention.ansi16, Ansi16::Red);
        assert_eq!(attention.xterm256, 46);
    }

    #[test]
    fn absent_outline_is_omitted() {
        let base = |tokens: &Tokens| {
            terminal_palette(tokens)
                .iter()
                .filter(|entry| !entry.name.starts_with("series_"))
                .count()
        };
        assert_eq!(base(&DARK), 34);
        assert_eq!(base(&LIGHT), 35);
        assert!(!terminal_palette(&DARK)
            .iter()
            .any(|entry| entry.name == "tab_active_line"));
        assert_eq!(terminal_palette(&LIGHT)[5].name, "tab_active_line");
    }

    #[cfg(feature = "series")]
    #[test]
    fn series_entries_follow_the_panel_in_order() {
        for tokens in [DARK, LIGHT] {
            let palette = terminal_palette(&tokens);
            let tail = &palette[palette.len() - 6..];
            for (i, entry) in tail.iter().enumerate() {
                assert_eq!(entry.name, format!("series_{}", i + 1));
                assert_eq!(entry.rgb, tokens.series[i]);
                assert_eq!(entry.xterm256, xterm256(tokens.series[i]));
            }
            assert_eq!(tail[0].ansi16, Ansi16::Blue);
            assert_eq!(tail[4].ansi16, Ansi16::Red);
        }
    }
}
