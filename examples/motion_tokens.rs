//! Exercise public motion durations and CSS easing samples.
use graphite_core::{
    capped_motion_duration, MotionCurve, MOTION_EMPHASIS, MOTION_MICRO, MOTION_STANDARD,
};
use std::time::Duration;

fn main() {
    println!(
        "durations={}ms/{}ms/{}ms cap={}ms",
        MOTION_MICRO.as_millis(),
        MOTION_STANDARD.as_millis(),
        MOTION_EMPHASIS.as_millis(),
        capped_motion_duration(Duration::from_secs(1)).as_millis()
    );
    for (curve, x, expected) in [
        (MotionCurve::EaseOut, 0.1, 0.15625),
        (MotionCurve::EaseOut, 0.2, 0.5),
        (MotionCurve::EaseIn, 0.325, 0.15625),
        (MotionCurve::EaseIn, 0.65, 0.5),
    ] {
        let actual = curve.apply(x);
        assert!((actual - expected).abs() < 0.00001);
        println!("{curve:?}({x})={actual:.5}");
    }
}
