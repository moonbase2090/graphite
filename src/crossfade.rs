//! Perceptual, allocation-free interpolation of complete themes.

use crate::{
    color_space::{from_oklab, oklab},
    Rgb, Tokens,
};
use std::time::Duration;

/// Theme crossfade duration.
pub const THEME_CROSSFADE_DURATION: Duration = Duration::from_millis(240);

/// Interpolate every color in OKLab. Requires the default-off `motion` feature.
///
/// Endpoints are exact. Progress is clamped; NaN selects the starting theme.
/// Variant and an outline present on only one side switch at halfway.
/// Interpolated colors are clipped to the sRGB gamut. Contrast floors are not
/// guaranteed during a transition; callers can snap with reduced motion.
#[must_use]
pub fn crossfade_tokens(from: &Tokens, to: &Tokens, progress: f32) -> Tokens {
    let t = if progress.is_nan() {
        0.0
    } else {
        progress.clamp(0.0, 1.0)
    };
    if t == 0.0 {
        return *from;
    }
    if t == 1.0 {
        return *to;
    }
    let discrete = if t < 0.5 { from } else { to };
    let color = |a, b| lerp_color(a, b, f64::from(t));
    Tokens {
        ground: color(from.ground, to.ground),
        bar: color(from.bar, to.bar),
        bar_line: color(from.bar_line, to.bar_line),
        divider: color(from.divider, to.divider),
        tab_active: color(from.tab_active, to.tab_active),
        tab_hover: color(from.tab_hover, to.tab_hover),
        text: color(from.text, to.text),
        text_strong: color(from.text_strong, to.text_strong),
        muted: color(from.muted, to.muted),
        tab_text: color(from.tab_text, to.tab_text),
        working: color(from.working, to.working),
        unseen: color(from.unseen, to.unseen),
        attention: color(from.attention, to.attention),
        on_attention: color(from.on_attention, to.on_attention),
        idle: color(from.idle, to.idle),
        field: color(from.field, to.field),
        field_line: color(from.field_line, to.field_line),
        key: color(from.key, to.key),
        key_line: color(from.key_line, to.key_line),
        key_text: color(from.key_text, to.key_text),
        status_bar: color(from.status_bar, to.status_bar),
        status_line: color(from.status_line, to.status_line),
        chip_active: color(from.chip_active, to.chip_active),
        separator: color(from.separator, to.separator),
        hairline: color(from.hairline, to.hairline),
        title_line: color(from.title_line, to.title_line),
        title_focus: color(from.title_focus, to.title_focus),
        title_focus_line: color(from.title_focus_line, to.title_focus_line),
        muted_focus: color(from.muted_focus, to.muted_focus),
        title_hover: color(from.title_hover, to.title_hover),
        hover_outline: color(from.hover_outline, to.hover_outline),
        unseen_text: color(from.unseen_text, to.unseen_text),
        cycle_head: color(from.cycle_head, to.cycle_head),
        panel: color(from.panel, to.panel),
        #[cfg(feature = "series")]
        series: std::array::from_fn(|i| color(from.series[i], to.series[i])),
        tab_active_line: match (from.tab_active_line, to.tab_active_line) {
            (Some(a), Some(b)) => Some(color(a, b)),
            _ => discrete.tab_active_line,
        },
        variant: discrete.variant,
    }
}

fn lerp_color(a: Rgb, b: Rgb, t: f64) -> Rgb {
    if a == b {
        return a;
    }
    let a = oklab(a);
    let b = oklab(b);
    from_oklab(std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ThemeVariant, DARK, LIGHT};
    #[test]
    fn endpoints_and_out_of_range_inputs_are_exact() {
        assert_eq!(THEME_CROSSFADE_DURATION.as_millis(), 240);
        assert_eq!(crossfade_tokens(&DARK, &LIGHT, 0.0), DARK);
        assert_eq!(crossfade_tokens(&DARK, &LIGHT, -1.0), DARK);
        assert_eq!(crossfade_tokens(&DARK, &LIGHT, f32::NAN), DARK);
        assert_eq!(crossfade_tokens(&DARK, &LIGHT, 1.0), LIGHT);
        assert_eq!(crossfade_tokens(&DARK, &LIGHT, 2.0), LIGHT);
        assert_eq!(crossfade_tokens(&DARK, &DARK, 0.5), DARK);
    }
    #[test]
    fn perceptual_midpoint_is_not_srgb_channel_average() {
        // Neutral OKLab L=0.5 has linear RGB=0.125, sRGB approximately 0.389.
        assert_eq!(lerp_color([0, 0, 0], [255, 255, 255], 0.5), [99, 99, 99]);
        assert_eq!(lerp_color([255, 0, 0], [0, 0, 255], 0.5), [140, 83, 162]);
        let mut from = DARK;
        let mut to = LIGHT;
        from.ground = [0, 0, 0];
        to.ground = [255, 255, 255];
        from.panel = [255, 0, 0];
        to.panel = [0, 0, 255];
        let mid = crossfade_tokens(&from, &to, 0.5);
        assert_eq!(mid.ground, [99, 99, 99]);
        assert_eq!(mid.panel, [140, 83, 162]);
    }
    #[test]
    fn optional_outline_and_variant_switch_at_halfway() {
        let early = crossfade_tokens(&DARK, &LIGHT, 0.499);
        assert_eq!(early.variant, ThemeVariant::Dark);
        assert_eq!(early.tab_active_line, None);
        let mid = crossfade_tokens(&DARK, &LIGHT, 0.5);
        assert_eq!(mid.variant, ThemeVariant::Light);
        assert_eq!(mid.tab_active_line, Some([0xaa, 0xb2, 0xbd]));
    }

    #[cfg(feature = "series")]
    #[test]
    fn categorical_series_colors_interpolate() {
        let mut from = DARK;
        let mut to = LIGHT;
        from.series[0] = [0, 0, 0];
        to.series[0] = [255, 255, 255];

        assert_eq!(crossfade_tokens(&from, &to, 0.5).series[0], [99, 99, 99]);
    }
}
