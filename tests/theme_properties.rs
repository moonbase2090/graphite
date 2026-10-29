//! Host-theme properties for backgrounds that share a dark or light polarity.
//! Arbitrary independent fills can make a shared ink's contrast floor impossible.
use graphite_core::{
    accent, contrast_ratio, derive_tokens, text_ink, Rgb, ThemeSource, ThemeVariant,
};
use proptest::prelude::*;

fn channel_color(light: bool) -> BoxedStrategy<Rgb> {
    let range = if light { 190u8..=255 } else { 0u8..=60 };
    (range.clone(), range.clone(), range)
        .prop_map(|(r, g, b)| [r, g, b])
        .boxed()
}

fn themes() -> BoxedStrategy<ThemeSource> {
    any::<bool>()
        .prop_flat_map(|light| {
            (
                channel_color(light),
                channel_color(!light),
                channel_color(light),
                channel_color(!light),
                channel_color(light),
                channel_color(light),
                any::<Rgb>(),
                any::<Rgb>(),
                any::<Rgb>(),
                any::<Rgb>(),
                any::<[Rgb; 16]>(),
            )
                .prop_map(
                    move |(
                        chrome_bg,
                        chrome_fg,
                        default_bg,
                        default_fg,
                        tab_active_bg,
                        pane_backdrop,
                        pane_border,
                        active_badge,
                        unseen_badge,
                        attention_badge,
                        ansi,
                    )| ThemeSource {
                        id: "generated-host".into(),
                        variant: if light {
                            ThemeVariant::Light
                        } else {
                            ThemeVariant::Dark
                        },
                        chrome_bg,
                        chrome_fg,
                        default_bg,
                        default_fg,
                        tab_active_bg,
                        pane_backdrop,
                        pane_border,
                        active_badge,
                        unseen_badge,
                        attention_badge,
                        ansi,
                    },
                )
        })
        .boxed()
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]
    #[test]
    fn derived_text_and_accent_meet_floors_for_coherent_hosts(source in themes()) {
        let t=derive_tokens(&source);
        for (name,ink,grounds) in [
            ("text",t.text,vec![t.bar,t.status_bar,t.tab_active,t.chip_active]),
            ("strong",t.text_strong,vec![t.bar,t.status_bar,t.tab_active,t.chip_active]),
            ("muted",t.muted,vec![t.bar,t.tab_active,t.field]),
            ("tab",t.tab_text,vec![t.bar,t.status_bar,t.tab_active,t.chip_active]),
            ("key",t.key_text,vec![t.key]),
            ("focus",t.muted_focus,vec![t.title_focus]),
            ("unseen",t.unseen_text,vec![t.bar,t.status_bar]),
            ("attention",t.on_attention,vec![t.attention]),
        ] {
            for ground in grounds { prop_assert!(contrast_ratio(ink,ground)>=4.5,"{} {:?} on {:?}",name,ink,ground); }
        }
        let focus=accent(&t,source.ansi[4]);
        prop_assert!(contrast_ratio(focus,t.bar)>=3.0,"accent {:?} on {:?}",focus,t.bar);
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 1024, ..ProptestConfig::default() })]
    #[test]
    fn text_ink_resolves_arbitrary_independent_surfaces(preferred in any::<Rgb>(), background in any::<Rgb>()) {
        let ink = text_ink(preferred, background);
        prop_assert!(contrast_ratio(ink, background) >= 4.5,
            "resolved ink {:?} on {:?} has ratio {}", ink, background, contrast_ratio(ink, background));
    }
}

#[test]
fn per_surface_text_ink_handles_conflicting_fills_independently() {
    let preferred = [128, 128, 128];
    let surfaces = [[0, 0, 0], [255, 255, 255], [118, 118, 118]];
    let resolved = surfaces.map(|background| text_ink(preferred, background));
    for (ink, background) in resolved.into_iter().zip(surfaces) {
        assert!(contrast_ratio(ink, background) >= 4.5);
    }
    assert_ne!(resolved[0], resolved[1]);
    assert_eq!(text_ink([20, 80, 190], [255, 255, 255]), [20, 80, 190]);
    assert!(text_ink([128; 3], [255; 3])[0] < 128);
    assert!(text_ink([80; 3], [0; 3])[0] > 80);
    let nearest = text_ink([40; 3], [117; 3]);
    assert!(contrast_ratio(nearest, [117; 3]) >= 4.5);
    assert!(
        nearest[0] < 100,
        "the closer dark interpolation should win: {nearest:?}"
    );
    assert!(contrast_ratio(text_ink([220; 3], [0; 3]), [0; 3]) >= 4.5);
}

#[test]
fn readable_attention_does_not_stop_short_of_the_floor() {
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
    assert!(
        contrast_ratio(t.on_attention, t.attention) >= 4.5,
        "ink {:?} ratio {}",
        t.on_attention,
        contrast_ratio(t.on_attention, t.attention)
    );
}
