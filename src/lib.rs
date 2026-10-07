//! Core types and algorithms shared by Graphite consumers.

mod color;
mod tokens;

pub use color::{
    accent, bar_color_name, bar_fills, contrast_ratio, mix_rgb, relative_luminance, step_bar_color,
    BarColor,
};
pub use tokens::{tokens, Rgb, ThemeVariant, Tokens, DARK, LIGHT};

#[cfg(test)]
mod tests {
    #[test]
    fn rust_version_is_pinned_to_1_85() {
        assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.85");
    }
}
