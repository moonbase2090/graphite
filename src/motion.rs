//! Easing and light-cycle progress mapping (renderer-agnostic).

use std::time::Duration;

/// Hover and focus transition duration.
pub const MOTION_MICRO: Duration = Duration::from_millis(80);
/// Value-change transition duration.
pub const MOTION_STANDARD: Duration = Duration::from_millis(160);
/// Open and close transition duration.
pub const MOTION_EMPHASIS: Duration = Duration::from_millis(240);
/// Maximum duration accepted by [`capped_motion_duration`].
pub const MOTION_MAX: Duration = Duration::from_millis(300);

/// Cap a caller-selected duration at the design maximum.
#[must_use]
pub fn capped_motion_duration(duration: Duration) -> Duration {
    duration.min(MOTION_MAX)
}

/// Easing curve applied to normalized progress in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotionCurve {
    /// Uniform progress along the ring.
    #[default]
    Linear,
    /// Smooth ease in and out (cubic).
    InOutCubic,
    /// Entering and value transitions, CSS cubic-bezier(0.2, 0, 0, 1).
    EaseOut,
    /// Exiting transitions, CSS cubic-bezier(0.4, 0, 1, 1).
    EaseIn,
}

impl MotionCurve {
    /// Maps `t` in `[0, 1]` through the curve.
    #[must_use]
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseOut => cubic_bezier(t, 0.2, 0.0),
            Self::EaseIn => cubic_bezier(t, 0.4, 1.0),
            Self::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

// Invert x(s) before evaluating y(s). Progress is the x coordinate, not the
// Bezier parameter. Both supported curves have y controls 0 and 1.
fn cubic_bezier(progress: f32, x1: f32, x2: f32) -> f32 {
    if progress <= 0.0 || progress >= 1.0 {
        return progress;
    }
    let (mut low, mut high) = (0.0, 1.0);
    for _ in 0..24 {
        let s = (low + high) * 0.5;
        let inv = 1.0 - s;
        let x = 3.0 * inv * inv * s * x1 + 3.0 * inv * s * s * x2 + s * s * s;
        if x < progress {
            low = s;
        } else {
            high = s;
        }
    }
    let s = (low + high) * 0.5;
    3.0 * (1.0 - s) * s * s + s * s * s
}

/// How many perimeter samples are traced at `progress` along a light-cycle sweep.
///
/// `progress` is elapsed / duration in `[0, 1]` while the sweep runs; values `>= 1`
/// mean the ring has settled. Duration and reduced-motion skipping stay in the host.
#[must_use]
pub fn light_cycle_traced_samples(progress: f32, sample_len: usize, curve: MotionCurve) -> usize {
    if sample_len == 0 {
        return 0;
    }
    let eased = curve.apply(progress.clamp(0.0, 1.0));
    (eased * sample_len as f32).round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_durations_and_cap_are_literal_milliseconds() {
        assert_eq!(MOTION_MICRO.as_millis(), 80);
        assert_eq!(MOTION_STANDARD.as_millis(), 160);
        assert_eq!(MOTION_EMPHASIS.as_millis(), 240);
        assert_eq!(capped_motion_duration(Duration::ZERO), Duration::ZERO);
        assert_eq!(
            capped_motion_duration(Duration::from_millis(240)).as_millis(),
            240
        );
        assert_eq!(
            capped_motion_duration(Duration::from_secs(2)).as_millis(),
            300
        );
    }

    #[test]
    fn bezier_samples_invert_the_time_coordinate() {
        // At parameter 1/4, ease-out has (x,y)=(0.1,0.15625).
        // At parameter 1/2 it has (0.2,0.5). Ease-in's x values differ.
        for (curve, x, y) in [
            (MotionCurve::EaseOut, 0.1, 0.15625),
            (MotionCurve::EaseOut, 0.2, 0.5),
            (MotionCurve::EaseIn, 0.325, 0.15625),
            (MotionCurve::EaseIn, 0.65, 0.5),
        ] {
            assert!((curve.apply(x) - y).abs() < 0.00001);
        }
    }

    #[test]
    fn bezier_curves_are_monotone_clamped_and_preserve_endpoints() {
        for curve in [MotionCurve::EaseOut, MotionCurve::EaseIn] {
            assert_eq!(curve.apply(-1.0), 0.0);
            assert_eq!(curve.apply(0.0), 0.0);
            assert_eq!(curve.apply(1.0), 1.0);
            assert_eq!(curve.apply(2.0), 1.0);
            let mut previous = 0.0;
            for step in 0..=1000 {
                let y = curve.apply(step as f32 / 1000.0);
                assert!((previous..=1.0).contains(&y));
                previous = y;
            }
        }
        assert_eq!(MotionCurve::default(), MotionCurve::Linear);
    }

    #[test]
    fn linear_maps_endpoints_and_midpoint() {
        assert_eq!(light_cycle_traced_samples(0.0, 100, MotionCurve::Linear), 0);
        assert_eq!(
            light_cycle_traced_samples(1.0, 100, MotionCurve::Linear),
            100
        );
        assert_eq!(
            light_cycle_traced_samples(0.5, 100, MotionCurve::Linear),
            50
        );
    }

    #[test]
    fn in_out_cubic_stays_inside_the_sample_range() {
        for step in 0..=20 {
            let t = step as f32 / 20.0;
            let traced = light_cycle_traced_samples(t, 80, MotionCurve::InOutCubic);
            assert!(traced <= 80);
        }
        assert_eq!(
            light_cycle_traced_samples(1.0, 80, MotionCurve::InOutCubic),
            80
        );
    }

    #[test]
    fn in_out_cubic_moves_slowly_at_the_start() {
        let early = light_cycle_traced_samples(0.25, 100, MotionCurve::InOutCubic);
        let linear = light_cycle_traced_samples(0.25, 100, MotionCurve::Linear);
        assert!(early < linear, "early {early} vs linear {linear}");
    }
}
