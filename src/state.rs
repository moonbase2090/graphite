//! Interaction state styling: focus, selection, disabled, error, hover.
//!
//! [`state_style`] returns colors and modifiers for a widget in one state at
//! one color depth. It is pure and allocation-free, so hosts resolve it once
//! per theme change and cache the result.

use crate::{Rgb, Tokens};

/// An interaction state from the design standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetState {
    /// No interaction.
    Normal,
    /// The pointer is over the widget.
    Hover,
    /// The widget has keyboard focus.
    Focus,
    /// The item is selected but the list is not focused.
    Selected,
    /// The item is selected and focused.
    SelectedFocus,
    /// The widget cannot be used. It never shows hover.
    Disabled,
    /// The widget holds an invalid value.
    Error,
}

/// How many colors the terminal can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorDepth {
    /// 24-bit color.
    TrueColor,
    /// The xterm 256-color palette. Colors are the same tokens; the consumer quantizes them.
    Ansi256,
    /// Sixteen ANSI colors. Fills are unreliable, so states use bold and reverse video.
    Ansi16,
}

/// Colors and modifiers for one state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateStyle {
    /// Primary text color.
    pub fg: Rgb,
    /// Secondary text color, such as captions.
    pub muted: Rgb,
    /// Fill behind the widget.
    pub bg: Rgb,
    /// Border or rule color.
    pub edge: Rgb,
    /// Draw the border Thick. Only focus uses this.
    pub thick: bool,
    /// Draw a `▌` marker at the left of the item, so state is never color alone.
    pub marker: bool,
    /// Bold text.
    pub bold: bool,
    /// Reverse video, used instead of fills at [`ColorDepth::Ansi16`].
    pub reverse: bool,
}

/// Resolves the style for `state` at `depth`.
///
/// `accent` is the focus color, normally `accent(tokens, DEFAULT_ACCENT)`.
pub fn state_style(
    tokens: &Tokens,
    accent: Rgb,
    state: WidgetState,
    depth: ColorDepth,
) -> StateStyle {
    let t = tokens;
    let base = StateStyle {
        fg: t.text,
        muted: t.muted,
        bg: t.panel,
        edge: t.hairline,
        thick: false,
        marker: false,
        bold: false,
        reverse: false,
    };
    let mut style = match state {
        WidgetState::Normal => base,
        WidgetState::Hover => StateStyle {
            bg: t.title_hover,
            edge: t.hover_outline,
            ..base
        },
        WidgetState::Focus => StateStyle {
            fg: t.text_strong,
            muted: t.muted_focus,
            bg: t.title_focus,
            edge: accent,
            thick: true,
            marker: true,
            bold: true,
            ..base
        },
        WidgetState::Selected => StateStyle {
            muted: t.muted_focus,
            bg: t.title_focus,
            ..base
        },
        WidgetState::SelectedFocus => StateStyle {
            muted: t.muted_focus,
            bg: t.title_focus,
            edge: accent,
            marker: true,
            ..base
        },
        WidgetState::Disabled => StateStyle {
            fg: t.muted,
            ..base
        },
        WidgetState::Error => StateStyle {
            edge: t.attention,
            muted: t.attention,
            ..base
        },
    };
    if depth == ColorDepth::Ansi16 {
        match state {
            WidgetState::Focus => style.reverse = true,
            WidgetState::Selected | WidgetState::SelectedFocus => style.reverse = true,
            _ => {}
        }
        // Fills do not survive a 16-color palette; reverse video carries the state.
        if style.reverse {
            style.bg = base.bg;
        }
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{accent, contrast_ratio, DARK, DEFAULT_ACCENT, LIGHT};

    const STATES: [WidgetState; 7] = [
        WidgetState::Normal,
        WidgetState::Hover,
        WidgetState::Focus,
        WidgetState::Selected,
        WidgetState::SelectedFocus,
        WidgetState::Disabled,
        WidgetState::Error,
    ];
    const DEPTHS: [ColorDepth; 3] = [
        ColorDepth::TrueColor,
        ColorDepth::Ansi256,
        ColorDepth::Ansi16,
    ];

    fn style(t: &Tokens, s: WidgetState, d: ColorDepth) -> StateStyle {
        state_style(t, accent(t, DEFAULT_ACCENT), s, d)
    }

    #[test]
    fn normal_uses_panel_text_and_hairline() {
        for t in [&DARK, &LIGHT] {
            for d in DEPTHS {
                let s = style(t, WidgetState::Normal, d);
                assert_eq!((s.fg, s.bg, s.edge), (t.text, t.panel, t.hairline));
                assert!(!s.bold && !s.reverse && !s.marker && !s.thick);
            }
        }
    }

    #[test]
    fn hover_uses_hover_fill_and_outline_and_no_modifiers() {
        for t in [&DARK, &LIGHT] {
            let s = style(t, WidgetState::Hover, ColorDepth::TrueColor);
            assert_eq!((s.bg, s.edge), (t.title_hover, t.hover_outline));
            assert!(!s.bold && !s.reverse && !s.thick && !s.marker);
        }
    }

    #[test]
    fn focus_is_accent_thick_bold_with_marker() {
        for t in [&DARK, &LIGHT] {
            let accent = accent(t, DEFAULT_ACCENT);
            for d in [ColorDepth::TrueColor, ColorDepth::Ansi256] {
                let s = style(t, WidgetState::Focus, d);
                assert_eq!((s.edge, s.bg, s.fg), (accent, t.title_focus, t.text_strong));
                assert!(s.thick && s.bold && s.marker && !s.reverse);
            }
            let s = style(t, WidgetState::Focus, ColorDepth::Ansi16);
            assert!(s.thick && s.bold && s.marker && s.reverse);
        }
    }

    #[test]
    fn selection_is_soft_fill_not_bold_and_reverse_at_sixteen() {
        for t in [&DARK, &LIGHT] {
            let s = style(t, WidgetState::Selected, ColorDepth::TrueColor);
            assert_eq!((s.bg, s.fg), (t.title_focus, t.text));
            assert!(!s.bold && !s.marker && !s.reverse);
            let s = style(t, WidgetState::Selected, ColorDepth::Ansi16);
            assert!(s.reverse && !s.bold && !s.marker);
            let s = style(t, WidgetState::SelectedFocus, ColorDepth::TrueColor);
            assert!(s.marker && !s.bold && !s.reverse && !s.thick);
            let s = style(t, WidgetState::SelectedFocus, ColorDepth::Ansi16);
            assert!(s.marker && s.reverse);
        }
    }

    #[test]
    fn disabled_is_muted_without_hover_and_error_uses_danger() {
        for t in [&DARK, &LIGHT] {
            for d in DEPTHS {
                let s = style(t, WidgetState::Disabled, d);
                assert_eq!((s.fg, s.bg, s.edge), (t.muted, t.panel, t.hairline));
                let s = style(t, WidgetState::Error, d);
                assert_eq!((s.edge, s.muted, s.fg), (t.attention, t.attention, t.text));
            }
        }
    }

    #[test]
    fn text_stays_readable_in_every_state_and_depth() {
        for t in [&DARK, &LIGHT] {
            for state in STATES {
                for d in DEPTHS {
                    let s = style(t, state, d);
                    assert!(
                        contrast_ratio(s.fg, s.bg) >= 4.5,
                        "fg {state:?} {d:?} {:?}",
                        t.variant
                    );
                    if state != WidgetState::Error {
                        assert!(
                            contrast_ratio(s.muted, s.bg) >= 4.5,
                            "muted {state:?} {d:?} {:?}",
                            t.variant
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn edges_that_mark_state_have_three_to_one_against_the_bar_or_fill() {
        for t in [&DARK, &LIGHT] {
            let s = style(t, WidgetState::Focus, ColorDepth::TrueColor);
            assert!(contrast_ratio(s.edge, t.bar) >= 3.0);
            let s = style(t, WidgetState::Error, ColorDepth::TrueColor);
            assert!(contrast_ratio(s.edge, t.panel) >= 3.0);
        }
    }
}
