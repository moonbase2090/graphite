//! Pure rounded-rectangle perimeter sampling for the pane light-cycle.

use crate::Rect;

/// Clockwise, approximately one-pixel samples around a rounded rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct RingSweep {
    /// Perimeter samples in clockwise order, beginning at the top-left arc.
    pub samples: Vec<(f32, f32)>,
}

impl RingSweep {
    /// Samples a rounded rectangle perimeter without painting or timing.
    pub fn for_slot(slot: Rect, radius: f32) -> Self {
        let _ = (slot, radius);
        todo!()
    }

    /// Returns the number of perimeter samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_sweep_starts_top_left_runs_clockwise_with_even_arc_steps() {
        let sweep = RingSweep::for_slot(Rect::new(0, 0, 100, 60), 8.0);
        assert!(
            (300..=312).contains(&sweep.len()),
            "arc-length total {}",
            sweep.len()
        );
        let samples = &sweep.samples;
        assert!((samples[0].0 - 8.0).abs() < 0.6 && samples[0].1.abs() < 0.6);
        assert!(samples[10].0 > samples[0].0 && samples[10].1.abs() < 0.6);
        let mut worst = 0.0f32;
        for pair in samples.windows(2) {
            let gap = ((pair[1].0 - pair[0].0).powi(2) + (pair[1].1 - pair[0].1).powi(2)).sqrt();
            worst = worst.max(gap);
        }
        assert!(worst <= 1.5, "uneven arc step {worst}");
        let right = samples
            .iter()
            .position(|&(x, _)| x > 99.0)
            .expect("reaches the right edge");
        let bottom = samples
            .iter()
            .position(|&(_, y)| y > 59.0)
            .expect("reaches the bottom edge");
        let left = samples
            .iter()
            .rposition(|&(x, _)| x < 1.0)
            .expect("returns up the left edge");
        assert!(right < bottom && bottom < left);
        let last = samples.last().unwrap();
        let home = ((last.0 - 8.0).powi(2) + last.1.powi(2)).sqrt();
        assert!(home < 2.0, "loop closes {last:?}");
    }
}
