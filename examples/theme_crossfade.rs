//! Exercise exact endpoints and independently known perceptual midpoints.
use graphite_core::{crossfade_tokens, DARK, LIGHT, THEME_CROSSFADE_DURATION};
fn main() {
    assert_eq!(crossfade_tokens(&DARK, &LIGHT, 0.0), DARK);
    assert_eq!(crossfade_tokens(&DARK, &LIGHT, 1.0), LIGHT);
    let mut from = DARK;
    let mut to = LIGHT;
    from.ground = [0, 0, 0];
    to.ground = [255, 255, 255];
    from.panel = [255, 0, 0];
    to.panel = [0, 0, 255];
    let mid = crossfade_tokens(&from, &to, 0.5);
    assert_eq!(mid.ground, [99, 99, 99]);
    assert_eq!(mid.panel, [140, 83, 162]);
    println!(
        "duration={}ms endpoints=exact neutral={:?} red-blue={:?}",
        THEME_CROSSFADE_DURATION.as_millis(),
        mid.ground,
        mid.panel
    );
}
