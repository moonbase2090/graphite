//! Graphite design tokens shared by consumers.

/// An sRGB color represented as red, green, and blue bytes.
pub type Rgb = [u8; 3];

/// The two token sets in the Graphite design brief.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    Dark,
    Light,
}

/// Renderer-independent Graphite colors and surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tokens {
    pub ground: Rgb,
    pub bar: Rgb,
    pub bar_line: Rgb,
    pub divider: Rgb,
    pub tab_active: Rgb,
    pub tab_active_line: Option<Rgb>,
    pub tab_hover: Rgb,
    pub text: Rgb,
    pub text_strong: Rgb,
    pub muted: Rgb,
    pub tab_text: Rgb,
    pub working: Rgb,
    pub unseen: Rgb,
    pub attention: Rgb,
    pub on_attention: Rgb,
    pub idle: Rgb,
    pub field: Rgb,
    pub field_line: Rgb,
    pub key: Rgb,
    pub key_line: Rgb,
    pub key_text: Rgb,
    pub status_bar: Rgb,
    pub status_line: Rgb,
    pub chip_active: Rgb,
    pub separator: Rgb,
    pub hairline: Rgb,
    pub title_line: Rgb,
    pub title_focus: Rgb,
    pub title_focus_line: Rgb,
    pub muted_focus: Rgb,
    pub title_hover: Rgb,
    pub hover_outline: Rgb,
    pub unseen_text: Rgb,
    pub cycle_head: Rgb,
    pub panel: Rgb,
    pub variant: ThemeVariant,
}

const fn rgb(hex: u32) -> Rgb {
    [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
}

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
    variant: ThemeVariant::Dark,
};

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
    variant: ThemeVariant::Light,
};

pub fn tokens(variant: ThemeVariant) -> &'static Tokens {
    match variant {
        ThemeVariant::Dark => &DARK,
        ThemeVariant::Light => &LIGHT,
    }
}

#[cfg(test)]
mod tests {
    use super::{Rgb, ThemeVariant, Tokens, DARK, LIGHT};

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
}
