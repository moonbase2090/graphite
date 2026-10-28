//! Text measurement seams and renderer-independent truncation.

use unicode_segmentation::UnicodeSegmentation;

/// The face used by a Graphite text measurement implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    /// Regular UI text.
    Regular,
    /// Emphasized UI text.
    SemiBold,
    /// Monospaced keycap text.
    Mono,
}

/// Measures text without coupling Graphite core to a font implementation.
pub trait TextMetrics {
    /// Returns the measured width in pixels.
    fn width(&self, face: Face, px: f32, text: &str) -> f32;
}

/// Truncates `text` to `max_w`, appending a Unicode ellipsis when needed.
///
/// Extended grapheme clusters are atomic. A combining mark, flag, ZWJ
/// sequence, or variation selector stays with its base, or the whole cluster
/// is dropped. If only the ellipsis fits, it is returned on its own. If even
/// the ellipsis does not fit, the result is empty.
pub fn ellipsize<M: TextMetrics + ?Sized>(
    metrics: &M,
    face: Face,
    px: f32,
    text: &str,
    max_w: f32,
) -> String {
    if metrics.width(face, px, text) <= max_w {
        return text.to_owned();
    }

    let ellipsis_width = metrics.width(face, px, "…");
    let mut out = String::new();
    for cluster in text.graphemes(true) {
        let mut next = out.clone();
        next.push_str(cluster);
        if metrics.width(face, px, &next) + ellipsis_width > max_w {
            break;
        }
        out = next;
    }

    let trimmed = out.trim_end();
    if trimmed.is_empty() && ellipsis_width > max_w {
        String::new()
    } else {
        format!("{trimmed}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OneCell;

    impl TextMetrics for OneCell {
        fn width(&self, _face: Face, _px: f32, text: &str) -> f32 {
            text.chars().count() as f32
        }
    }

    #[test]
    fn full_family_stays_atomic_and_narrow_budget_still_marks_truncation() {
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "👨‍👩‍👧‍👦abcd", 2.0), "…");
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "👨‍👩‍👧‍👦abcd", 8.0),
            "👨‍👩‍👧‍👦…"
        );
    }

    #[test]
    fn budget_smaller_than_the_ellipsis_stays_empty() {
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "🇺🇸abcd", 0.5), "");
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "abcdef", 0.0), "");
    }

    #[test]
    fn short_text_is_unchanged() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "short", 8.0),
            "short"
        );
    }

    #[test]
    fn exact_fit_is_unchanged() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "four", 4.0),
            "four"
        );
    }

    #[test]
    fn cut_keeps_a_scalar_prefix_and_ellipsis() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "abcdef", 4.0),
            "abc…"
        );
    }

    #[test]
    fn flag_that_cannot_fit_with_the_ellipsis_is_dropped_whole() {
        // Two regional indicators. A scalar walk returns "🇺…".
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "🇺🇸abcd", 2.0), "…");
    }

    #[test]
    fn flag_is_kept_whole_with_the_ellipsis() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "🇺🇸abcd", 3.0),
            "🇺🇸…"
        );
    }

    #[test]
    fn flag_after_text_is_not_split() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "ab🇺🇸cd", 4.0),
            "ab…"
        );
    }

    #[test]
    fn flag_that_fits_is_unchanged() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "🇺🇸ab", 4.0),
            "🇺🇸ab"
        );
    }

    #[test]
    fn zwj_sequence_that_cannot_fit_with_the_ellipsis_is_dropped_whole() {
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "👨‍👩abcd", 3.0), "…");
    }

    #[test]
    fn zwj_sequence_is_kept_whole_with_the_ellipsis() {
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "👨‍👩abcd", 4.0),
            "👨‍👩…"
        );
    }

    #[test]
    fn zwj_sequence_after_text_is_not_split() {
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "ab👨‍👩", 4.0), "ab…");
    }

    #[test]
    fn zwj_sequence_with_variation_selectors_stays_whole() {
        let couple = "👨\u{FE0F}\u{200D}👩\u{FE0F}xy";
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, couple, 4.0), "…");
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, couple, 6.0),
            "👨\u{FE0F}\u{200D}👩\u{FE0F}…"
        );
    }

    #[test]
    fn combining_mark_stays_with_its_base() {
        // U+0301. A scalar walk at width 2 returns "a…".
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "a\u{301}bc", 2.0),
            "…"
        );
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "a\u{301}bc", 3.0),
            "a\u{301}…"
        );
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "xa\u{301}bc", 3.0),
            "x…"
        );
    }

    #[test]
    fn variation_selector_is_not_left_behind() {
        // U+2764 U+FE0F. A scalar walk at width 2 returns "❤…".
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "❤️abcd", 2.0), "…");
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "❤️abcd", 3.0),
            "❤️…"
        );
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, "ab❤️cd", 4.0),
            "ab…"
        );
    }
}
