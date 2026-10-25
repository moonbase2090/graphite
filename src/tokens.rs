//! Graphite design tokens shared by consumers.

/// An sRGB color represented as red, green, and blue bytes.
pub type Rgb = [u8; 3];

/// The two token sets in the Graphite design brief.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// The dark Graphite token set.
    Dark,
    /// The light Graphite token set.
    Light,
}

/// Renderer-independent Graphite colors and surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tokens {
    /// Window ground behind the bars and panes.
    pub ground: Rgb,
    /// Tabs-bar fill.
    pub bar: Rgb,
    /// Hairline along the tabs bar.
    pub bar_line: Rgb,
    /// Rule between the Space dropdown and the first tab.
    pub divider: Rgb,
    /// Active tab chip fill.
    pub tab_active: Rgb,
    /// Optional outline around the active tab chip on light themes.
    pub tab_active_line: Option<Rgb>,
    /// Hover fill for tabs and other tab-bar controls.
    pub tab_hover: Rgb,
    /// Primary text color.
    pub text: Rgb,
    /// Strong text color for emphasized labels.
    pub text_strong: Rgb,
    /// Muted text color.
    pub muted: Rgb,
    /// Tab-label text color.
    pub tab_text: Rgb,
    /// Working-status color.
    pub working: Rgb,
    /// Unseen-output color.
    pub unseen: Rgb,
    /// Attention-status color.
    pub attention: Rgb,
    /// Text color placed on an attention-status fill.
    pub on_attention: Rgb,
    /// Quiet or idle-status color.
    pub idle: Rgb,
    /// Command-field and pane-field fill.
    pub field: Rgb,
    /// Command-field outline.
    pub field_line: Rgb,
    /// Keyboard-keycap fill.
    pub key: Rgb,
    /// Keyboard-keycap outline.
    pub key_line: Rgb,
    /// Keyboard-keycap text.
    pub key_text: Rgb,
    /// Spaces-bar fill.
    pub status_bar: Rgb,
    /// Hairline along the spaces bar.
    pub status_line: Rgb,
    /// Active Space chip fill.
    pub chip_active: Rgb,
    /// Separator between status counts.
    pub separator: Rgb,
    /// Pane outline.
    pub hairline: Rgb,
    /// Pane title-row rule.
    pub title_line: Rgb,
    /// Focused pane title-row fill.
    pub title_focus: Rgb,
    /// Focused pane title-row outline.
    pub title_focus_line: Rgb,
    /// Muted text on the focused title row.
    pub muted_focus: Rgb,
    /// Handle hover fill behind the header dot and name.
    pub title_hover: Rgb,
    /// Hover outline around a pane whose handle is hovered.
    pub hover_outline: Rgb,
    /// Unseen or mail status text color.
    pub unseen_text: Rgb,
    /// Light-cycle vehicle-head color.
    pub cycle_head: Rgb,
    /// Dialog and overlay card fill, also used for pane previews.
    pub panel: Rgb,
    /// Categorical series colors for charts, in fixed order series-1..6.
    #[cfg(feature = "series")]
    pub series: [Rgb; 6],
    /// Theme variant from which these tokens come.
    pub variant: ThemeVariant,
}

const fn rgb(hex: u32) -> Rgb {
    [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
}

#[cfg(feature = "series")]
/// Dark series palette in order series-1..6: blue `#5AA9FF`, green `#4CC98A`, amber
/// `#F2B84B`, violet `#C08BFF`, coral `#FF7A6B`, teal `#3FD0D0`. Each has at least 3:1
/// contrast against the ground, bar, panel, and field surfaces.
pub const SERIES_DARK: [Rgb; 6] = [
    rgb(0x5aa9ff),
    rgb(0x4cc98a),
    rgb(0xf2b84b),
    rgb(0xc08bff),
    rgb(0xff7a6b),
    rgb(0x3fd0d0),
];

#[cfg(feature = "series")]
/// Light series palette in order series-1..6: `#1F6FD1`, `#17855A`, `#9A6200`, `#7B45C9`,
/// `#C2392B`, `#0D7F86`. The same hues as the dark palette, darkened to keep 3:1 contrast.
pub const SERIES_LIGHT: [Rgb; 6] = [
    rgb(0x1f6fd1),
    rgb(0x17855a),
    rgb(0x9a6200),
    rgb(0x7b45c9),
    rgb(0xc2392b),
    rgb(0x0d7f86),
];

/// Dark Graphite tokens from the design brief.
pub const DARK: Tokens = Tokens {
    ground: rgb(0x101216),
    bar: rgb(0x15181d),
    bar_line: rgb(0x23272e),
    divider: rgb(0x2b3038),
    tab_active: rgb(0x252a33),
    tab_active_line: None,
    tab_hover: rgb(0x1f232a),
    text: rgb(0xe6e9ee),
    text_strong: rgb(0xf2f4f7),
    muted: rgb(0xa3abb8),
    tab_text: rgb(0xaeb5c1),
    working: rgb(0x4cc98a),
    unseen: rgb(0xf2b84b),
    attention: rgb(0xff7a6b),
    on_attention: rgb(0x1a0d0b),
    idle: rgb(0x6b7380),
    field: rgb(0x111317),
    field_line: rgb(0x2b3038),
    key: rgb(0x1f232a),
    key_line: rgb(0x2f343d),
    key_text: rgb(0xc6ccd6),
    status_bar: rgb(0x0d0f12),
    status_line: rgb(0x1e2228),
    chip_active: rgb(0x1f232a),
    separator: rgb(0x3a404a),
    hairline: rgb(0x262a32),
    title_line: rgb(0x22262d),
    title_focus: rgb(0x1c2330),
    title_focus_line: rgb(0x2a3140),
    muted_focus: rgb(0xb4bcc9),
    title_hover: rgb(0x252a33),
    hover_outline: rgb(0x3d5f8f),
    unseen_text: rgb(0xf2b84b),
    cycle_head: rgb(0xd6e8ff),
    panel: rgb(0x181b21),
    #[cfg(feature = "series")]
    series: SERIES_DARK,
    variant: ThemeVariant::Dark,
};

/// Light Graphite tokens from the design brief.
pub const LIGHT: Tokens = Tokens {
    ground: rgb(0xe9ecf0),
    bar: rgb(0xeef0f3),
    bar_line: rgb(0xd5d9df),
    divider: rgb(0xd5d9df),
    tab_active: rgb(0xffffff),
    tab_active_line: Some(rgb(0xaab2bd)),
    tab_hover: rgb(0xe2e6eb),
    text: rgb(0x1f2329),
    text_strong: rgb(0x1f2329),
    muted: rgb(0x5b6472),
    tab_text: rgb(0x4a525e),
    working: rgb(0x1f8a55),
    unseen: rgb(0xb87a0a),
    attention: rgb(0xc2392b),
    on_attention: rgb(0xffffff),
    idle: rgb(0x8a929e),
    field: rgb(0xffffff),
    field_line: rgb(0xc9ced6),
    key: rgb(0xf4f6f8),
    key_line: rgb(0xc9ced6),
    key_text: rgb(0x1f2329),
    status_bar: rgb(0xe4e7ec),
    status_line: rgb(0xd5d9df),
    chip_active: rgb(0xffffff),
    separator: rgb(0xa9b0ba),
    hairline: rgb(0xd5d9df),
    title_line: rgb(0xe3e6ea),
    title_focus: rgb(0xeaf1fc),
    title_focus_line: rgb(0xcddcf3),
    muted_focus: rgb(0x4a525e),
    title_hover: rgb(0xe2e6eb),
    hover_outline: rgb(0x9dbbe8),
    unseen_text: rgb(0x9a6200),
    cycle_head: rgb(0x163f80),
    panel: rgb(0xffffff),
    #[cfg(feature = "series")]
    series: SERIES_LIGHT,
    variant: ThemeVariant::Light,
};

/// Return the static token set for a theme variant.
pub fn tokens(variant: ThemeVariant) -> &'static Tokens {
    match variant {
        ThemeVariant::Dark => &DARK,
        ThemeVariant::Light => &LIGHT,
    }
}

#[cfg(test)]
mod tests {
    use super::{Rgb, ThemeVariant, Tokens, DARK, LIGHT};
    #[cfg(feature = "series")]
    use super::{SERIES_DARK, SERIES_LIGHT};

    const fn rgb(hex: u32) -> Rgb {
        [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
    }

    fn expected_dark() -> Tokens {
        Tokens {
            ground: rgb(0x101216),
            bar: rgb(0x15181d),
            bar_line: rgb(0x23272e),
            divider: rgb(0x2b3038),
            tab_active: rgb(0x252a33),
            tab_active_line: None,
            tab_hover: rgb(0x1f232a),
            text: rgb(0xe6e9ee),
            text_strong: rgb(0xf2f4f7),
            muted: rgb(0xa3abb8),
            tab_text: rgb(0xaeb5c1),
            working: rgb(0x4cc98a),
            unseen: rgb(0xf2b84b),
            attention: rgb(0xff7a6b),
            on_attention: rgb(0x1a0d0b),
            idle: rgb(0x6b7380),
            field: rgb(0x111317),
            field_line: rgb(0x2b3038),
            key: rgb(0x1f232a),
            key_line: rgb(0x2f343d),
            key_text: rgb(0xc6ccd6),
            status_bar: rgb(0x0d0f12),
            status_line: rgb(0x1e2228),
            chip_active: rgb(0x1f232a),
            separator: rgb(0x3a404a),
            hairline: rgb(0x262a32),
            title_line: rgb(0x22262d),
            title_focus: rgb(0x1c2330),
            title_focus_line: rgb(0x2a3140),
            muted_focus: rgb(0xb4bcc9),
            title_hover: rgb(0x252a33),
            hover_outline: rgb(0x3d5f8f),
            unseen_text: rgb(0xf2b84b),
            cycle_head: rgb(0xd6e8ff),
            panel: rgb(0x181b21),
            #[cfg(feature = "series")]
            series: SERIES_DARK,
            variant: ThemeVariant::Dark,
        }
    }

    fn expected_light() -> Tokens {
        Tokens {
            ground: rgb(0xe9ecf0),
            bar: rgb(0xeef0f3),
            bar_line: rgb(0xd5d9df),
            divider: rgb(0xd5d9df),
            tab_active: rgb(0xffffff),
            tab_active_line: Some(rgb(0xaab2bd)),
            tab_hover: rgb(0xe2e6eb),
            text: rgb(0x1f2329),
            text_strong: rgb(0x1f2329),
            muted: rgb(0x5b6472),
            tab_text: rgb(0x4a525e),
            working: rgb(0x1f8a55),
            unseen: rgb(0xb87a0a),
            attention: rgb(0xc2392b),
            on_attention: rgb(0xffffff),
            idle: rgb(0x8a929e),
            field: rgb(0xffffff),
            field_line: rgb(0xc9ced6),
            key: rgb(0xf4f6f8),
            key_line: rgb(0xc9ced6),
            key_text: rgb(0x1f2329),
            status_bar: rgb(0xe4e7ec),
            status_line: rgb(0xd5d9df),
            chip_active: rgb(0xffffff),
            separator: rgb(0xa9b0ba),
            hairline: rgb(0xd5d9df),
            title_line: rgb(0xe3e6ea),
            title_focus: rgb(0xeaf1fc),
            title_focus_line: rgb(0xcddcf3),
            muted_focus: rgb(0x4a525e),
            title_hover: rgb(0xe2e6eb),
            hover_outline: rgb(0x9dbbe8),
            unseen_text: rgb(0x9a6200),
            cycle_head: rgb(0x163f80),
            panel: rgb(0xffffff),
            #[cfg(feature = "series")]
            series: SERIES_LIGHT,
            variant: ThemeVariant::Light,
        }
    }

    #[test]
    fn dark_tokens_pin_every_baseline_field() {
        assert_eq!(DARK, expected_dark());
    }

    #[test]
    fn light_tokens_pin_every_baseline_field() {
        assert_eq!(LIGHT, expected_light());
    }

    #[cfg(feature = "series")]
    #[test]
    fn series_have_three_to_one_contrast_on_every_surface() {
        for t in [&DARK, &LIGHT] {
            for (n, color) in t.series.iter().enumerate() {
                for surface in [t.ground, t.bar, t.panel, t.field, t.tab_active] {
                    let ratio = crate::contrast_ratio(*color, surface);
                    assert!(ratio >= 3.0, "series-{} {ratio:.2} on {surface:?}", n + 1);
                }
            }
        }
    }

    #[cfg(feature = "series")]
    #[test]
    fn series_are_distinct_and_documented_values() {
        for palette in [SERIES_DARK, SERIES_LIGHT] {
            for (i, a) in palette.iter().enumerate() {
                for b in &palette[i + 1..] {
                    assert_ne!(a, b);
                }
            }
        }
        assert_eq!(SERIES_DARK[0], [0x5a, 0xa9, 0xff]);
        assert_eq!(SERIES_LIGHT[0], [0x1f, 0x6f, 0xd1]);
    }
}
