//! Easing and light-cycle progress mapping (renderer-agnostic).

/// Easing curve applied to normalized progress in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotionCurve {
    /// Uniform progress along the ring.
    #[default]
    Linear,
    /// Smooth ease in and out (cubic).
    InOutCubic,
}

impl MotionCurve {
    /// Maps `t` in `[0, 1]` through the curve.
    #[must_use]
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
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
    fn linear_maps_endpoints_and_midpoint() {
        assert_eq!(light_cycle_traced_samples(0.0, 100, MotionCurve::Linear), 0);
        assert_eq!(light_cycle_traced_samples(1.0, 100, MotionCurve::Linear), 100);
        assert_eq!(light_cycle_traced_samples(0.5, 100, MotionCurve::Linear), 50);
    }

    #[test]
    fn in_out_cubic_stays_inside_the_sample_range() {
        for step in 0..=20 {
            let t = step as f32 / 20.0;
            let traced = light_cycle_traced_samples(t, 80, MotionCurve::InOutCubic);
            assert!(traced <= 80);
        }
        assert_eq!(light_cycle_traced_samples(1.0, 80, MotionCurve::InOutCubic), 80);
    }

    #[test]
    fn in_out_cubic_moves_slowly_at_the_start() {
        let early = light_cycle_traced_samples(0.25, 100, MotionCurve::InOutCubic);
        let linear = light_cycle_traced_samples(0.25, 100, MotionCurve::Linear);
        assert!(early < linear, "early {early} vs linear {linear}");
    }
}
