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
///
/// Grapheme clusters are atomic. A flag, a ZWJ sequence, or a variation
/// selector stays with its base, or the whole cluster is dropped.
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
    for cluster in grapheme_clusters(text) {
        let mut next = out.clone();
        next.push_str(cluster);
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

/// Yields extended clusters for the sequences `ellipsize` must not split:
/// regional-indicator pairs, variation selectors, emoji modifiers, and ZWJ joins.
fn grapheme_clusters(text: &str) -> GraphemeClusters<'_> {
    GraphemeClusters { rest: text }
}

struct GraphemeClusters<'a> {
    rest: &'a str,
}

impl<'a> Iterator for GraphemeClusters<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut end = 0;
        let mut chars = self.rest.chars();
        let first = chars.next()?;
        end += first.len_utf8();
        if is_regional_indicator(first) {
            if let Some(second) = chars.next() {
                if is_regional_indicator(second) {
                    end += second.len_utf8();
                }
            }
        }

        loop {
            let tail = &self.rest[end..];
            let Some(next) = tail.chars().next() else {
                break;
            };
            if attaches_to_previous(next) {
                end += next.len_utf8();
                continue;
            }
            if is_zwj(next) {
                let after_zwj = &tail[next.len_utf8()..];
                let Some(base) = after_zwj.chars().next() else {
                    break;
                };
                end += next.len_utf8() + base.len_utf8();
                if is_regional_indicator(base) {
                    if let Some(second) = after_zwj[base.len_utf8()..].chars().next() {
                        if is_regional_indicator(second) {
                            end += second.len_utf8();
                        }
                    }
                }
                continue;
            }
            break;
        }

        let (cluster, rest) = self.rest.split_at(end);
        self.rest = rest;
        Some(cluster)
    }
}

fn is_regional_indicator(ch: char) -> bool {
    ('\u{1F1E6}'..='\u{1F1FF}').contains(&ch)
}

fn is_variation_selector(ch: char) -> bool {
    ('\u{FE00}'..='\u{FE0F}').contains(&ch) || ('\u{E0100}'..='\u{E01EF}').contains(&ch)
}

fn is_emoji_modifier(ch: char) -> bool {
    ('\u{1F3FB}'..='\u{1F3FF}').contains(&ch)
}

fn is_zwj(ch: char) -> bool {
    ch == '\u{200D}'
}

fn attaches_to_previous(ch: char) -> bool {
    is_variation_selector(ch) || is_emoji_modifier(ch)
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

    #[test]
    fn flag_that_cannot_fit_with_the_ellipsis_is_dropped_whole() {
        // Two regional indicators. A scalar walk returns "🇺…".
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "🇺🇸abcd", 2.0), "");
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
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "👨‍👩abcd", 3.0), "");
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
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, couple, 4.0), "");
        assert_eq!(
            ellipsize(&OneCell, Face::Regular, 12.0, couple, 6.0),
            "👨\u{FE0F}\u{200D}👩\u{FE0F}…"
        );
    }

    #[test]
    fn variation_selector_is_not_left_behind() {
        // U+2764 U+FE0F. A scalar walk at width 2 returns "❤…".
        assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "❤️abcd", 2.0), "");
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
