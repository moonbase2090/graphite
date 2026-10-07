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

/// Return whether a source is one of the unchanged built-in Graphite themes.
pub fn uses_brief(_source: &ThemeSource) -> bool {
    false
}

/// Derive Graphite tokens from a theme source.
pub fn derive_tokens(source: &ThemeSource) -> Tokens {
    crate::tokens::tokens(source.variant).to_owned()
}

/// Return brief tokens verbatim, or derive tokens for another theme.
pub fn theme_tokens(source: &ThemeSource) -> Tokens {
    derive_tokens(source)
}

/// Apply an optional bar preset to theme-derived tokens.
pub fn bar_tokens(source: &ThemeSource, _bar: Option<BarColor>) -> Tokens {
    derive_tokens(source)
}

#[cfg(test)]
mod tests {
    use super::{bar_tokens, derive_tokens, theme_tokens, uses_brief, ThemeSource};
    use crate::{bar_fills, contrast_ratio, BarColor, ThemeVariant, DARK, LIGHT};

    const fn rgb(hex: u32) -> [u8; 3] {
        [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
    }

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
