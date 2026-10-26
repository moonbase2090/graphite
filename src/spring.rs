//! Spring motion and a frame-rate governor for animated chrome (renderer-agnostic).

use std::time::Duration;

/// Spring stiffness of the design default.
pub const SPRING_STIFFNESS: f64 = 300.0;
/// Spring damping of the design default.
pub const SPRING_DAMPING: f64 = 30.0;
/// Spring mass of the design default.
pub const SPRING_MASS: f64 = 1.0;
/// A spring is settled once it is within this many cells of its target.
pub const SETTLE_CELLS: f64 = 0.125;

/// Integration step. Frame times are split into steps of this size so the result does not depend on the frame rate.
const STEP: Duration = Duration::from_millis(1);

/// A damped spring that moves a value (in cells) toward a target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    position: f64,
    velocity: f64,
    target: f64,
}

impl Spring {
    /// Creates a spring at rest on `start`.
    #[must_use]
    pub fn new(start: f64) -> Self {
        Self {
            position: start,
            velocity: 0.0,
            target: start,
        }
    }

    /// Sets the value the spring moves toward.
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Current position.
    #[must_use]
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Returns true once the position is within [`SETTLE_CELLS`] of the target.
    #[must_use]
    pub fn settled(&self) -> bool {
        (self.position - self.target).abs() < SETTLE_CELLS
    }

    /// Advances the spring by `dt`. With `reduced_motion`, the spring snaps to its target.
    ///
    /// The caller reads any reduced-motion setting; this crate takes it as an input.
    pub fn step(&mut self, dt: Duration, reduced_motion: bool) -> f64 {
        if reduced_motion {
            self.position = self.target;
            self.velocity = 0.0;
            return self.position;
        }
        let mut remaining = dt;
        while remaining > Duration::ZERO {
            let h = remaining.min(STEP).as_secs_f64();
            remaining = remaining.saturating_sub(STEP);
            let acceleration = (-SPRING_STIFFNESS * (self.position - self.target)
                - SPRING_DAMPING * self.velocity)
                / SPRING_MASS;
            self.velocity += acceleration * h;
            self.position += self.velocity * h;
        }
        self.position
    }
}

/// Lowers the frame rate after two consecutive frames miss the budget, and reports the rate to draw at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameGovernor {
    budget: Duration,
    over_budget_streak: u32,
    degraded: bool,
}

/// Frame rate the governor asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    /// 60 frames per second.
    Full,
    /// 30 frames per second.
    Half,
}

impl FrameRate {
    /// The frame interval for this rate.
    #[must_use]
    pub fn interval(self) -> Duration {
        match self {
            Self::Full => Duration::from_nanos(16_666_667),
            Self::Half => Duration::from_nanos(33_333_333),
        }
    }
}

impl FrameGovernor {
    /// Creates a governor with the design frame budget of 16 ms.
    #[must_use]
    pub fn new() -> Self {
        Self::with_budget(Duration::from_millis(16))
    }

    /// Creates a governor with a custom budget.
    #[must_use]
    pub fn with_budget(budget: Duration) -> Self {
        Self {
            budget,
            over_budget_streak: 0,
            degraded: false,
        }
    }

    /// Records one frame's work time and returns the rate to use from the next frame.
    pub fn record(&mut self, frame_time: Duration) -> FrameRate {
        if frame_time > self.budget {
            self.over_budget_streak += 1;
            if self.over_budget_streak >= 2 {
                self.degraded = true;
            }
        } else {
            self.over_budget_streak = 0;
        }
        self.rate()
    }

    /// The rate in effect now.
    #[must_use]
    pub fn rate(&self) -> FrameRate {
        if self.degraded {
            FrameRate::Half
        } else {
            FrameRate::Full
        }
    }

    /// Returns to 60 frames per second, for example after the host restores full motion.
    pub fn reset(&mut self) {
        self.over_budget_streak = 0;
        self.degraded = false;
    }
}

impl Default for FrameGovernor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_until_settled(spring: &mut Spring, frame: Duration) -> Duration {
        let mut elapsed = Duration::ZERO;
        while !spring.settled() && elapsed < Duration::from_secs(5) {
            spring.step(frame, false);
            elapsed += frame;
        }
        elapsed
    }

    #[test]
    fn design_constants_are_pinned() {
        assert_eq!(SPRING_STIFFNESS, 300.0);
        assert_eq!(SPRING_DAMPING, 30.0);
        assert_eq!(SPRING_MASS, 1.0);
        assert_eq!(SETTLE_CELLS, 0.125);
    }

    #[test]
    fn spring_settles_within_half_a_second_from_ten_cells() {
        let mut spring = Spring::new(0.0);
        spring.set_target(10.0);
        let settle = run_until_settled(&mut spring, Duration::from_millis(16));
        assert!(
            settle < Duration::from_millis(500),
            "settled after {settle:?}"
        );
    }

    #[test]
    fn overshoot_stays_below_one_eighth_cell() {
        let mut spring = Spring::new(0.0);
        spring.set_target(10.0);
        let mut peak: f64 = 0.0;
        for _ in 0..600 {
            peak = peak.max(spring.step(Duration::from_millis(1), false));
        }
        assert!(peak - 10.0 < SETTLE_CELLS, "overshoot {}", peak - 10.0);
    }

    #[test]
    fn step_size_does_not_change_the_result() {
        let mut fast = Spring::new(0.0);
        let mut slow = Spring::new(0.0);
        fast.set_target(5.0);
        slow.set_target(5.0);
        for _ in 0..30 {
            fast.step(Duration::from_millis(8), false);
        }
        for _ in 0..60 {
            slow.step(Duration::from_millis(4), false);
        }
        assert!((fast.position() - slow.position()).abs() < 0.01);
    }

    #[test]
    fn reduced_motion_snaps_to_the_target() {
        let mut spring = Spring::new(0.0);
        spring.set_target(7.0);
        assert_eq!(spring.step(Duration::from_millis(16), true), 7.0);
        assert!(spring.settled());
    }

    #[test]
    fn a_spring_at_rest_on_its_target_stays_put() {
        let mut spring = Spring::new(3.0);
        assert!(spring.settled());
        assert_eq!(spring.step(Duration::from_secs(1), false), 3.0);
    }

    #[test]
    fn one_slow_frame_does_not_degrade_the_rate() {
        let mut governor = FrameGovernor::new();
        assert_eq!(governor.record(Duration::from_millis(40)), FrameRate::Full);
        assert_eq!(governor.record(Duration::from_millis(5)), FrameRate::Full);
    }

    #[test]
    fn two_consecutive_slow_frames_degrade_to_thirty() {
        let mut governor = FrameGovernor::new();
        governor.record(Duration::from_millis(20));
        assert_eq!(governor.record(Duration::from_millis(20)), FrameRate::Half);
        assert_eq!(governor.rate().interval(), Duration::from_nanos(33_333_333));
    }

    #[test]
    fn reset_restores_full_rate() {
        let mut governor = FrameGovernor::with_budget(Duration::from_millis(1));
        governor.record(Duration::from_millis(5));
        governor.record(Duration::from_millis(5));
        governor.reset();
        assert_eq!(governor.rate(), FrameRate::Full);
    }
}
