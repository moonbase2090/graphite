//! Text measurement seams and renderer-independent truncation.

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
    for ch in text.chars() {
        let mut next = out.clone();
        next.push(ch);
        if metrics.width(face, px, &next) + ellipsis_width > max_w {
            break;
        }
        out = next;
    }

    let trimmed = out.trim_end();
    if trimmed.is_empty() {
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
}
