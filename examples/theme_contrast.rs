//! Exercise the public theme-derivation contrast floor on the reduced regression.
use graphite_core::{contrast_ratio, derive_tokens, ThemeSource, ThemeVariant};
fn main() {
    let source = ThemeSource {
        id: "attention-floor".into(),
        variant: ThemeVariant::Dark,
        chrome_bg: [0; 3],
        chrome_fg: [190; 3],
        default_bg: [0; 3],
        default_fg: [190; 3],
        tab_active_bg: [0; 3],
        pane_backdrop: [0; 3],
        pane_border: [0; 3],
        active_badge: [0; 3],
        unseen_badge: [0; 3],
        attention_badge: [221, 51, 81],
        ansi: [[0; 3]; 16],
    };
    let t = derive_tokens(&source);
    let ratio = contrast_ratio(t.on_attention, t.attention);
    assert!(ratio >= 4.5);
    println!(
        "attention={:?} ink={:?} contrast={ratio:.5} floor=4.5",
        t.attention, t.on_attention
    );
}
