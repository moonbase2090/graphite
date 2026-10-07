//! Pure legend-run measurement and atomic packing.

use crate::{Face, Rect, TextMetrics};

const KEY_TEXT: f32 = 11.0;
const KEY_PAD_X: f32 = 6.0;
const KEY_H: f32 = 18.0;

/// An already-resolved legend run supplied by the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegendRun {
    /// Text without keycaps.
    Text(String),
    /// Atomic keycaps followed by an optional caption.
    Keys { caps: Vec<String>, caption: String },
}

/// A keycap rectangle and its already-resolved label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendChip {
    /// Physical keycap rectangle.
    pub rect: Rect,
    /// Already-resolved key label.
    pub label: String,
}

/// Pure result of packing the visible legend runs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LegendPaint {
    /// Keycaps from the runs that fit atomically.
    pub chips: Vec<LegendChip>,
    /// Captions from the runs that fit atomically.
    pub captions: Vec<String>,
}

/// Packs already-resolved legend runs, dropping whole groups that do not fit.
pub fn legend_keys<M: TextMetrics + ?Sized>(
    metrics: &M,
    scale_milli: u32,
    runs: &[LegendRun],
    width: usize,
    bottom: usize,
    rows: usize,
) -> LegendPaint {
    let _ = (metrics, scale_milli, runs, width, bottom, rows);
    todo!()
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

    fn runs() -> [LegendRun; 2] {
        [
            LegendRun::Keys {
                caps: vec!["C-S-[".to_owned(), "C-S-]".to_owned()],
                caption: "color".to_owned(),
            },
            LegendRun::Keys {
                caps: vec!["C-S-B".to_owned()],
                caption: "bars".to_owned(),
            },
        ]
    }

    #[test]
    fn legend_run_that_fits_keeps_every_group() {
        let paint = legend_keys(&OneCell, 1000, &runs(), 120, 48, 1);
        assert_eq!(
            paint
                .chips
                .iter()
                .map(|chip| chip.label.as_str())
                .collect::<Vec<_>>(),
            vec!["C-S-[", "C-S-]", "C-S-B"]
        );
        assert_eq!(paint.captions, vec!["color", "bars"]);
        assert_eq!(paint.chips[0].rect, Rect::new(12, 15, 17, 18));
        assert_eq!(paint.chips[2].rect, Rect::new(73, 15, 17, 18));
    }

    #[test]
    fn legend_packer_drops_a_shortcut_group_that_does_not_fit() {
        let paint = legend_keys(&OneCell, 1000, &runs(), 80, 48, 1);
        assert_eq!(
            paint
                .chips
                .iter()
                .map(|chip| chip.label.as_str())
                .collect::<Vec<_>>(),
            vec!["C-S-[", "C-S-]"]
        );
        assert_eq!(paint.captions, vec!["color"]);
    }
}
