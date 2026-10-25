//! Print the modifiers every state resolves to, per color depth, through the public API.
use graphite_core::{accent, state_style, ColorDepth, WidgetState, DARK, DEFAULT_ACCENT};

fn main() {
    let focus = accent(&DARK, DEFAULT_ACCENT);
    for depth in [
        ColorDepth::TrueColor,
        ColorDepth::Ansi256,
        ColorDepth::Ansi16,
    ] {
        for state in [
            WidgetState::Normal,
            WidgetState::Hover,
            WidgetState::Focus,
            WidgetState::Selected,
            WidgetState::SelectedFocus,
            WidgetState::Disabled,
            WidgetState::Error,
        ] {
            let s = state_style(&DARK, focus, state, depth);
            println!(
                "{depth:?} {state:?}: thick={} marker={} bold={} reverse={}",
                s.thick, s.marker, s.bold, s.reverse
            );
        }
    }
}
