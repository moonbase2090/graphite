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
    Keys {
        /// Already-resolved key labels in display order.
        caps: Vec<String>,
        /// Optional caption after the keycaps.
        caption: String,
    },
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

struct LegendMetrics {
    key_px: f32,
    label_px: f32,
    pad: f32,
    key_h: usize,
    chip_gap: f32,
    caption_gap: f32,
    group_gap: f32,
}

fn legend_metrics(scale_milli: u32, bar_h: usize) -> LegendMetrics {
    let scale = scale_milli as f32 / 1000.0;
    LegendMetrics {
        key_px: KEY_TEXT * scale,
        label_px: crate::scale_px(11.0, scale_milli).max(9) as f32,
        pad: KEY_PAD_X * scale,
        key_h: crate::scale_px(KEY_H, scale_milli).clamp(1, bar_h.max(1)),
        chip_gap: 4.0 * scale,
        caption_gap: 6.0 * scale,
        group_gap: 12.0 * scale,
    }
}

fn legend_chip_width<M: TextMetrics + ?Sized>(metrics: &M, label: &str, m: &LegendMetrics) -> f32 {
    (metrics.width(Face::Mono, m.key_px, label) + 2.0 * m.pad)
        .ceil()
        .max((m.pad * 2.0).ceil())
}

fn legend_run_width<M: TextMetrics + ?Sized>(
    metrics: &M,
    run: &LegendRun,
    m: &LegendMetrics,
) -> f32 {
    match run {
        LegendRun::Text(text) if text.is_empty() => 0.0,
        LegendRun::Text(text) => metrics.width(Face::Regular, m.label_px, text),
        LegendRun::Keys { caps, caption } => {
            if caps.is_empty() {
                return 0.0;
            }
            let mut width = 0.0;
            for (index, cap) in caps.iter().enumerate() {
                if index > 0 {
                    width += m.chip_gap;
                }
                width += legend_chip_width(metrics, cap, m);
            }
            if !caption.is_empty() {
                width += m.caption_gap + metrics.width(Face::Regular, m.label_px, caption);
            }
            width
        }
    }
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
    if width == 0 || bottom == 0 || rows == 0 {
        return LegendPaint::default();
    }
    let row_h = crate::scale_px(22.0, scale_milli).max(16);
    let bar_h = row_h.saturating_mul(rows).min(bottom);
    let y = bottom.saturating_sub(bar_h);
    let m = legend_metrics(scale_milli, bar_h);
    let key_y = y + bar_h.saturating_sub(m.key_h) / 2;
    let limit = width.saturating_sub(crate::scale_px(8.0, scale_milli)) as f32;
    let mut pen = crate::scale_px(12.0, scale_milli) as f32;
    let mut started = false;
    let mut result = LegendPaint::default();
    for run in runs {
        let run_width = legend_run_width(metrics, run, &m);
        if run_width <= 0.0 {
            continue;
        }
        let start = if started { pen + m.group_gap } else { pen };
        if start + run_width > limit {
            break;
        }
        if let LegendRun::Keys { caps, caption } = run {
            let mut chip_pen = start;
            for (index, cap) in caps.iter().enumerate() {
                if index > 0 {
                    chip_pen += m.chip_gap;
                }
                let chip_w = legend_chip_width(metrics, cap, &m) as usize;
                result.chips.push(LegendChip {
                    rect: Rect::new(chip_pen.round() as usize, key_y, chip_w, m.key_h),
                    label: cap.clone(),
                });
                chip_pen += chip_w as f32;
            }
            if !caption.is_empty() {
                result.captions.push(caption.clone());
            }
        }
        pen = start + run_width;
        started = true;
    }
    result
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
        assert_eq!(paint.chips[0].rect, Rect::new(12, 28, 17, 18));
        assert_eq!(paint.chips[2].rect, Rect::new(73, 28, 17, 18));
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
