//! Export shared web theme CSS custom properties from `graphite-core`.
//!
//! Default dark and light tokens come from `graphite_core::DARK` and `LIGHT`.
//! Pod dashboard themes are derived through `graphite_core::theme_tokens`
//! (which delegates to `derive_tokens`), with readable ink and focus colors
//! resolved via `graphite_core::text_ink` and `graphite_core::accent`.

use std::fmt::Write as _;

use graphite_core::{
    accent, contrast_ratio, mix_rgb, text_ink, theme_tokens, Rgb, ThemeSource, ThemeVariant,
    Tokens, DARK, LIGHT, MOTION_EMPHASIS, MOTION_MICRO, MOTION_STANDARD, PANE_GAP, PANE_PAD,
    PANE_RADIUS,
};

const WHITE: Rgb = [0xff, 0xff, 0xff];
const DARK_ON_ACCENT_PREF: Rgb = [0x0f, 0x17, 0x1d];

const fn rgb(hex: u32) -> Rgb {
    [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8]
}

fn hex(c: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

/// Seed inputs for one pod dashboard theme passed to `graphite_core::theme_tokens`.
pub struct ThemeSeed {
    pub id: &'static str,
    pub dark_accent: Rgb,
    pub light_accent: Rgb,
    pub dark_ground: Rgb,
    pub dark_surface: Rgb,
    pub dark_raised: Rgb,
    pub dark_line: Rgb,
    pub swatch_secondary: Rgb,
}

const INDIGO_DARK_ACCENT: Rgb = rgb(0xb0befe);
const INDIGO_LIGHT_ACCENT: Rgb = rgb(0x1f5a9c);

const THEMES: &[ThemeSeed] = &[
    ThemeSeed {
        id: "mono-slate",
        dark_accent: rgb(0x94a3b8),
        light_accent: rgb(0x546074),
        dark_ground: rgb(0x0f1214),
        dark_surface: rgb(0x1d2226),
        dark_raised: rgb(0x23292d),
        dark_line: rgb(0x424c55),
        swatch_secondary: rgb(0x64748b),
    },
    ThemeSeed {
        id: "forest-mint",
        dark_accent: rgb(0x5eead4),
        light_accent: rgb(0x388582),
        dark_ground: rgb(0x0c1515),
        dark_surface: rgb(0x182929),
        dark_raised: rgb(0x1d3130),
        dark_line: rgb(0x33605c),
        swatch_secondary: rgb(0x99f6e4),
    },
    ThemeSeed {
        id: "mono-sand",
        dark_accent: rgb(0xd6b98c),
        light_accent: rgb(0x766b5d),
        dark_ground: rgb(0x121311),
        dark_surface: rgb(0x242422),
        dark_raised: rgb(0x2b2b28),
        dark_line: rgb(0x545248),
        swatch_secondary: rgb(0xa68b5b),
    },
    ThemeSeed {
        id: "analog-azure",
        dark_accent: rgb(0x38bdf8),
        light_accent: rgb(0x246d95),
        dark_ground: rgb(0x0a1317),
        dark_surface: rgb(0x14252d),
        dark_raised: rgb(0x182c35),
        dark_line: rgb(0x285367),
        swatch_secondary: rgb(0x0ea5e9),
    },
    ThemeSeed {
        id: "midnight-indigo",
        dark_accent: rgb(0x818cf8),
        light_accent: rgb(0x4a5495),
        dark_ground: rgb(0x0e1117),
        dark_surface: rgb(0x1b202d),
        dark_raised: rgb(0x212635),
        dark_line: rgb(0x3d4567),
        swatch_secondary: rgb(0xa78bfa),
    },
    ThemeSeed {
        id: "comp-coral",
        dark_accent: rgb(0x2dd4bf),
        light_accent: rgb(0x1f7977),
        dark_ground: rgb(0x0a1414),
        dark_surface: rgb(0x132727),
        dark_raised: rgb(0x172f2e),
        dark_line: rgb(0x255a57),
        swatch_secondary: rgb(0xfb7185),
    },
];

/// Web CSS role mapping table from Graphite `Tokens` + `accent`/`text_ink`.
///
/// Every Comms color variable derives from one row of this table:
/// - `--ground`         <- `tokens.ground`
/// - `--surface`        <- `tokens.panel`
/// - `--surface-raised` <- `tokens.tab_active`
/// - `--surface-sunken` <- `tokens.status_bar`
/// - `--line`           <- `tokens.hairline`
/// - `--line-strong`    <- `tokens.idle` (dark) / `tokens.separator` (light)
/// - `--ink`            <- `text_ink(tokens.text, tokens.panel)`
/// - `--ink-muted`      <- `text_ink(tokens.muted, tokens.panel)`
/// - `--ink-faint`      <- `tokens.idle`
/// - `--accent`         <- `accent(&tokens, focus_seed)`
/// - `--accent-ink`     <- `text_ink(accent, tokens.panel)`
/// - `--on-accent`      <- `text_ink(preferred_on_accent, accent)`
/// - `--verified`       <- `text_ink(tokens.working, tokens.panel)`
/// - `--caution`        <- `text_ink(tokens.unseen, tokens.panel)`
/// - `--fault`          <- `text_ink(tokens.attention, tokens.panel)`
/// - `--shadow-rgb`     <- `tokens.status_bar` (dark) / `tokens.text` (light)
/// - `--backdrop-rgb`   <- `tokens.status_bar` (dark) / `tokens.text` (light)
pub struct WebPalette {
    pub ground: Rgb,
    pub surface: Rgb,
    pub surface_raised: Rgb,
    pub surface_sunken: Rgb,
    pub line: Rgb,
    pub line_strong: Rgb,
    pub ink: Rgb,
    pub ink_muted: Rgb,
    pub ink_faint: Rgb,
    pub accent: Rgb,
    pub accent_ink: Rgb,
    pub on_accent: Rgb,
    pub verified: Rgb,
    pub caution: Rgb,
    pub fault: Rgb,
    pub caution_tint: Rgb,
    pub fault_tint: Rgb,
    pub shadow_rgb: Rgb,
    pub shadow_alpha: &'static str,
    pub backdrop_rgb: Rgb,
    pub backdrop_alpha: &'static str,
}

fn map_tokens(tokens: &Tokens, focus_seed: Rgb) -> WebPalette {
    let light = tokens.variant == ThemeVariant::Light;
    let focus_accent = accent(tokens, focus_seed);
    let accent_ink = text_ink(focus_accent, tokens.panel);
    // On light surfaces, anchor the solid accent fill to `accent_ink` so white
    // button text (`--on-accent`) always clears WCAG AA (4.5:1) without shifting
    // the visible hue (e.g. Forest Mint #388582 -> #37827f).
    let resolved_accent = if light { accent_ink } else { focus_accent };
    let on_accent_pref = if light { WHITE } else { DARK_ON_ACCENT_PREF };
    let on_accent = text_ink(on_accent_pref, resolved_accent);
    let ink = text_ink(tokens.text, tokens.panel);
    let ink_muted = text_ink(tokens.muted, tokens.panel);
    let verified = text_ink(tokens.working, tokens.panel);
    let caution = text_ink(tokens.unseen, tokens.panel);
    let fault = text_ink(tokens.attention, tokens.panel);
    let line_strong = if light { tokens.separator } else { tokens.idle };
    let tint_weight = if light { 30 } else { 42 };
    let caution_tint = mix_rgb(tokens.panel, tokens.unseen, tint_weight);
    let fault_tint = mix_rgb(tokens.panel, tokens.attention, tint_weight);
    let (shadow_rgb, shadow_alpha, backdrop_rgb, backdrop_alpha) = if light {
        (tokens.text, ".16", tokens.text, ".45")
    } else {
        (tokens.status_bar, ".55", tokens.status_bar, ".68")
    };

    WebPalette {
        ground: tokens.ground,
        surface: tokens.panel,
        surface_raised: tokens.tab_active,
        surface_sunken: tokens.status_bar,
        line: tokens.hairline,
        line_strong,
        ink,
        ink_muted,
        ink_faint: tokens.idle,
        accent: resolved_accent,
        accent_ink,
        on_accent,
        verified,
        caution,
        fault,
        caution_tint,
        fault_tint,
        shadow_rgb,
        shadow_alpha,
        backdrop_rgb,
        backdrop_alpha,
    }
}

fn dashboard_source(seed: &ThemeSeed, variant: ThemeVariant) -> ThemeSource {
    let base = match variant {
        ThemeVariant::Dark => DARK,
        ThemeVariant::Light => LIGHT,
    };
    let focus = match variant {
        ThemeVariant::Dark => seed.dark_accent,
        ThemeVariant::Light => seed.light_accent,
    };
    let mut ansi = [base.text; 16];
    ansi[4] = focus;

    match variant {
        ThemeVariant::Dark => ThemeSource {
            id: seed.id.to_owned(),
            variant,
            chrome_bg: seed.dark_surface,
            chrome_fg: base.text,
            default_bg: seed.dark_surface,
            default_fg: base.text,
            tab_active_bg: seed.dark_raised,
            pane_backdrop: seed.dark_ground,
            pane_border: seed.dark_line,
            active_badge: base.working,
            unseen_badge: base.unseen,
            attention_badge: base.attention,
            ansi,
        },
        ThemeVariant::Light => {
            let ground = mix_rgb(base.ground, focus, 18);
            let bar = mix_rgb(base.bar, focus, 14);
            let border = mix_rgb(base.hairline, focus, 36);
            ThemeSource {
                id: seed.id.to_owned(),
                variant,
                chrome_bg: bar,
                chrome_fg: base.text,
                default_bg: base.panel,
                default_fg: base.text,
                tab_active_bg: base.tab_active,
                pane_backdrop: ground,
                pane_border: border,
                active_badge: base.working,
                unseen_badge: base.unseen,
                attention_badge: base.attention,
                ansi,
            }
        }
    }
}

fn control_edge(line: Rgb, ink: Rgb, surfaces: &[Rgb]) -> Rgb {
    let clears_floor = |candidate| {
        surfaces
            .iter()
            .all(|surface| contrast_ratio(candidate, *surface) >= 3.0)
    };
    if clears_floor(line) {
        return line;
    }
    for weight in 1..=256 {
        let candidate = mix_rgb(line, ink, weight);
        if clears_floor(candidate) {
            return candidate;
        }
    }
    ink
}

/// The Comms-provided dashboard theme seeds. Indigo and graphite are built-in palettes.
pub fn themes() -> &'static [ThemeSeed] {
    THEMES
}

/// Resolve a named theme for a dark or light scheme.
pub fn palette(theme: &str, variant: ThemeVariant) -> Option<WebPalette> {
    let (tokens, focus) = match theme {
        "graphite" => (
            if variant == ThemeVariant::Dark {
                DARK
            } else {
                LIGHT
            },
            graphite_core::DEFAULT_ACCENT,
        ),
        "indigo" => (
            if variant == ThemeVariant::Dark {
                DARK
            } else {
                LIGHT
            },
            if variant == ThemeVariant::Dark {
                INDIGO_DARK_ACCENT
            } else {
                INDIGO_LIGHT_ACCENT
            },
        ),
        id => {
            let seed = THEMES.iter().find(|seed| seed.id == id)?;
            (
                theme_tokens(&dashboard_source(seed, variant)),
                if variant == ThemeVariant::Dark {
                    seed.dark_accent
                } else {
                    seed.light_accent
                },
            )
        }
    };
    Some(map_tokens(&tokens, focus))
}

fn write_palette_block(out: &mut String, indent: &str, selector: &str, p: &WebPalette) {
    let _ = writeln!(out, "{indent}{selector} {{");
    let _ = writeln!(out, "{indent}  --gr-ground: {};", hex(p.ground));
    let _ = writeln!(out, "{indent}  --gr-surface: {};", hex(p.surface));
    let _ = writeln!(
        out,
        "{indent}  --gr-surface-raised: {};",
        hex(p.surface_raised)
    );
    let _ = writeln!(
        out,
        "{indent}  --gr-surface-sunken: {};",
        hex(p.surface_sunken)
    );
    let _ = writeln!(out, "{indent}  --gr-line: {};", hex(p.line));
    let edge = control_edge(
        p.line_strong,
        p.ink,
        &[p.ground, p.surface, p.surface_raised],
    );
    let _ = writeln!(out, "{indent}  --gr-line-strong: {};", hex(p.line_strong));
    let _ = writeln!(out, "{indent}  --gr-control-edge: {};", hex(edge));
    let _ = writeln!(out, "{indent}  --gr-ink: {};", hex(p.ink));
    let _ = writeln!(out, "{indent}  --gr-ink-muted: {};", hex(p.ink_muted));
    let _ = writeln!(out, "{indent}  --gr-ink-faint: {};", hex(p.ink_faint));
    let _ = writeln!(out, "{indent}  --gr-accent: {};", hex(p.accent));
    let _ = writeln!(out, "{indent}  --gr-accent-ink: {};", hex(p.accent_ink));
    let _ = writeln!(out, "{indent}  --gr-on-accent: {};", hex(p.on_accent));
    let _ = writeln!(out, "{indent}  --gr-verified: {};", hex(p.verified));
    let _ = writeln!(out, "{indent}  --gr-caution: {};", hex(p.caution));
    let _ = writeln!(out, "{indent}  --gr-fault: {};", hex(p.fault));
    let _ = writeln!(out, "{indent}  --gr-caution-tint: {};", hex(p.caution_tint));
    let _ = writeln!(out, "{indent}  --gr-fault-tint: {};", hex(p.fault_tint));
    let _ = writeln!(
        out,
        "{indent}  --gr-shadow-sheet: 0 12px 32px rgb({} {} {} / {});",
        p.shadow_rgb[0], p.shadow_rgb[1], p.shadow_rgb[2], p.shadow_alpha
    );
    let _ = writeln!(
        out,
        "{indent}  --gr-backdrop: rgb({} {} {} / {});",
        p.backdrop_rgb[0], p.backdrop_rgb[1], p.backdrop_rgb[2], p.backdrop_alpha
    );
    let scheme = if p.surface == LIGHT.panel {
        "light"
    } else {
        "dark"
    };
    let _ = writeln!(out, "{indent}  color-scheme: {scheme};");
    let _ = writeln!(out, "{indent}}}");
}

pub fn stylesheet() -> String {
    let mut out = String::from("/* Generated by graphite-css. Regenerate with `cargo run -p graphite-css -- --write`. */\n\n");
    let _ = writeln!(out, ":root {{");
    let _ = writeln!(
        out,
        "  --gr-motion-micro: {}ms;\n  --gr-motion-standard: {}ms;\n  --gr-motion-emphasis: {}ms;\n  --gr-radius: {}px;\n  --gr-gap: {}px;\n  --gr-pad: {}px;",
        MOTION_MICRO.as_millis(), MOTION_STANDARD.as_millis(), MOTION_EMPHASIS.as_millis(),
        PANE_RADIUS, PANE_GAP.0, PANE_PAD.0
    );
    let indigo_dark = map_tokens(&DARK, INDIGO_DARK_ACCENT);
    let indigo_light = map_tokens(&LIGHT, INDIGO_LIGHT_ACCENT);
    let graphite_dark = map_tokens(&DARK, graphite_core::DEFAULT_ACCENT);
    let graphite_light = map_tokens(&LIGHT, graphite_core::DEFAULT_ACCENT);
    let _ = writeln!(
        out,
        "  --gr-swatch-graphite-a: {};\n  --gr-swatch-graphite-b: {};",
        hex(graphite_dark.accent),
        hex(graphite_light.accent)
    );
    let _ = writeln!(
        out,
        "  --gr-swatch-indigo-a: {};\n  --gr-swatch-indigo-b: {};",
        hex(indigo_dark.accent),
        hex(indigo_light.accent)
    );
    for seed in THEMES {
        let tokens = theme_tokens(&dashboard_source(seed, ThemeVariant::Dark));
        let _ = writeln!(
            out,
            "  --gr-swatch-{}-a: {};\n  --gr-swatch-{}-b: {};",
            seed.id,
            hex(accent(&tokens, seed.dark_accent)),
            seed.id,
            hex(seed.swatch_secondary)
        );
    }
    let _ = writeln!(out, "}}\n");
    // Graphite is the unset-theme fallback. Theme selectors below are more specific.
    write_palette_block(
        &mut out,
        "",
        ":root:not([data-theme])",
        &map_tokens(&DARK, graphite_core::DEFAULT_ACCENT),
    );
    out.push_str("@media (prefers-color-scheme: light) {\n");
    write_palette_block(
        &mut out,
        "  ",
        ":root:not([data-theme]):not([data-scheme=\"dark\"])",
        &map_tokens(&LIGHT, graphite_core::DEFAULT_ACCENT),
    );
    out.push_str("}\n");
    write_palette_block(
        &mut out,
        "",
        ":root:not([data-theme])[data-scheme=\"light\"]",
        &map_tokens(&LIGHT, graphite_core::DEFAULT_ACCENT),
    );
    for (id, dark, light) in std::iter::once((
        "graphite",
        map_tokens(&DARK, graphite_core::DEFAULT_ACCENT),
        map_tokens(&LIGHT, graphite_core::DEFAULT_ACCENT),
    ))
    .chain(std::iter::once(("indigo", indigo_dark, indigo_light)))
    .chain(THEMES.iter().map(|seed| {
        (
            seed.id,
            map_tokens(
                &theme_tokens(&dashboard_source(seed, ThemeVariant::Dark)),
                seed.dark_accent,
            ),
            map_tokens(
                &theme_tokens(&dashboard_source(seed, ThemeVariant::Light)),
                seed.light_accent,
            ),
        )
    })) {
        let base = format!(":root[data-theme=\"{id}\"]");
        write_palette_block(&mut out, "", &base, &dark);
        out.push_str("@media (prefers-color-scheme: light) {\n");
        write_palette_block(
            &mut out,
            "  ",
            &format!("{base}:not([data-scheme=\"dark\"])"),
            &light,
        );
        out.push_str("}\n");
        write_palette_block(
            &mut out,
            "",
            &format!("{base}[data-scheme=\"light\"]"),
            &light,
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_css_uses_prefixed_roles_and_scheme_overrides() {
        let css = stylesheet();
        assert!(css.contains("--gr-control-edge:"));
        assert!(css.contains(":root[data-theme=\"indigo\"][data-scheme=\"light\"]"));
        assert!(!css.contains("--surface:"));
    }
    #[test]
    fn light_control_edge_clears_three_to_one() {
        let p = map_tokens(&LIGHT, INDIGO_LIGHT_ACCENT);
        let edge = control_edge(
            p.line_strong,
            p.ink,
            &[p.ground, p.surface, p.surface_raised],
        );
        for surface in [p.ground, p.surface, p.surface_raised] {
            assert!(contrast_ratio(edge, surface) >= 3.0);
        }
    }
    #[test]
    fn generated_file_is_current() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../web/tokens/graphite-tokens.css");
        assert_eq!(std::fs::read_to_string(path).unwrap(), stylesheet());
        let package_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/ui/tokens.css");
        assert_eq!(std::fs::read_to_string(package_path).unwrap(), stylesheet());
    }

    #[test]
    fn every_exported_theme_meets_text_and_control_contrast() {
        for theme in [
            "graphite",
            "indigo",
            "mono-slate",
            "forest-mint",
            "mono-sand",
            "analog-azure",
            "midnight-indigo",
            "comp-coral",
        ] {
            for variant in [ThemeVariant::Dark, ThemeVariant::Light] {
                let p = palette(theme, variant).unwrap();
                for (fg, bg) in [
                    (p.ink, p.surface),
                    (p.ink_muted, p.surface),
                    (p.accent_ink, p.surface),
                    (p.on_accent, p.accent),
                    (p.verified, p.surface),
                    (p.caution, p.surface),
                    (p.fault, p.surface),
                ] {
                    assert!(
                        contrast_ratio(fg, bg) >= 4.5,
                        "{theme} {variant:?} text contrast"
                    );
                }
                assert!(
                    contrast_ratio(p.ink, p.surface_raised) >= 4.5,
                    "{theme} {variant:?} raised-surface ink"
                );
                for bg in [p.ground, p.surface, p.surface_raised] {
                    assert!(
                        contrast_ratio(p.accent, bg) >= 3.0,
                        "{theme} {variant:?} focus accent on {bg:?}"
                    );
                }
                assert!(
                    [p.ground, p.surface, p.surface_raised]
                        .into_iter()
                        .all(|surface| {
                            contrast_ratio(
                                control_edge(
                                    p.line_strong,
                                    p.ink,
                                    &[p.ground, p.surface, p.surface_raised],
                                ),
                                surface,
                            ) >= 3.0
                        }),
                    "{theme} {variant:?} control edge"
                );
            }
        }
    }
}
