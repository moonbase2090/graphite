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
        let (x0, y0) = (slot.x as f32, slot.y as f32);
        let (x1, y1) = (slot.right() as f32, (slot.y + slot.h) as f32);
        let r = radius.min((x1 - x0) / 2.0).min((y1 - y0) / 2.0).max(0.0);
        let mut samples = Vec::new();
        use std::f32::consts::{FRAC_PI_2, PI};
        push_straight(&mut samples, x0 + r, y0, x1 - r, y0);
        push_arc(&mut samples, x1 - r, y0 + r, r, -FRAC_PI_2, 0.0);
        push_straight(&mut samples, x1, y0 + r, x1, y1 - r);
        push_arc(&mut samples, x1 - r, y1 - r, r, 0.0, FRAC_PI_2);
        push_straight(&mut samples, x1 - r, y1, x0 + r, y1);
        push_arc(&mut samples, x0 + r, y1 - r, r, FRAC_PI_2, PI);
        push_straight(&mut samples, x0, y1 - r, x0, y0 + r);
        push_arc(&mut samples, x0 + r, y0 + r, r, PI, 3.0 * FRAC_PI_2);
        Self { samples }
    }

    /// Returns the number of perimeter samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether the perimeter has no samples.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

fn push_straight(samples: &mut Vec<(f32, f32)>, ax: f32, ay: f32, bx: f32, by: f32) {
    let len = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
    if len < 0.5 {
        return;
    }
    let steps = len.round() as usize;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        samples.push((ax + (bx - ax) * t, ay + (by - ay) * t));
    }
}

fn push_arc(samples: &mut Vec<(f32, f32)>, cx: f32, cy: f32, r: f32, a0: f32, a1: f32) {
    let steps = ((a1 - a0).abs() * r).round().max(1.0) as usize;
    for i in 0..steps {
        let a = a0 + (a1 - a0) * i as f32 / steps as f32;
        samples.push((cx + r * a.cos(), cy + r * a.sin()));
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
