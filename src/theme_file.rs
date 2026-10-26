//! TOML theme files for [`Tokens`] (requires the `serde` feature).
//!
//! A theme file is a flat table. `variant` is `"dark"` or `"light"`, every
//! other key is a token name with a `"#RRGGBB"` value, and `tab_active_line`
//! may be omitted. Unknown keys are rejected so typos fail loudly.

use crate::{Rgb, ThemeVariant, Tokens};
use std::fmt;
use toml::{Table, Value};

/// Why a theme file could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeFileError {
    /// The text is not valid TOML.
    Syntax(String),
    /// A required key is absent.
    Missing(&'static str),
    /// A key is not a token name.
    Unknown(String),
    /// A value is not a string, or not `#RRGGBB`.
    BadColor {
        /// The token name.
        key: String,
        /// What the file contained.
        value: String,
    },
    /// `variant` is not `"dark"` or `"light"`.
    BadVariant(String),
}

impl fmt::Display for ThemeFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(e) => write!(f, "theme file is not valid TOML: {e}"),
            Self::Missing(key) => write!(f, "theme file is missing `{key}`"),
            Self::Unknown(key) => write!(f, "theme file has unknown key `{key}`"),
            Self::BadColor { key, value } => {
                write!(f, "`{key}` must be a color like \"#1A2B3C\", found {value}")
            }
            Self::BadVariant(v) => {
                write!(f, "`variant` must be \"dark\" or \"light\", found {v}")
            }
        }
    }
}

impl std::error::Error for ThemeFileError {}

/// Calls `$m!` with the name of every required color token.
macro_rules! required_colors {
    ($m:ident) => {
        $m!(
            ground,
            bar,
            bar_line,
            divider,
            tab_active,
            tab_hover,
            text,
            text_strong,
            muted,
            tab_text,
            working,
            unseen,
            attention,
            on_attention,
            idle,
            field,
            field_line,
            key,
            key_line,
            key_text,
            status_bar,
            status_line,
            chip_active,
            separator,
            hairline,
            title_line,
            title_focus,
            title_focus_line,
            muted_focus,
            title_hover,
            hover_outline,
            unseen_text,
            cycle_head,
            panel
        )
    };
}

macro_rules! names {
    ($($name:ident),*) => { &[$(stringify!($name)),*] };
}

const OPTIONAL: &str = "tab_active_line";

fn hex(c: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2])
}

fn parse_hex(key: &str, value: &Value) -> Result<Rgb, ThemeFileError> {
    let bad = || ThemeFileError::BadColor {
        key: key.to_owned(),
        value: value.to_string(),
    };
    let text = value.as_str().ok_or_else(bad)?;
    let digits = text.strip_prefix('#').ok_or_else(bad)?;
    if digits.len() != 6 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(bad());
    }
    let n = u32::from_str_radix(digits, 16).map_err(|_| bad())?;
    Ok([(n >> 16) as u8, (n >> 8) as u8, n as u8])
}

impl Tokens {
    /// Serializes these tokens as a theme file.
    pub fn to_toml(&self) -> String {
        let mut table = Table::new();
        let variant = match self.variant {
            ThemeVariant::Dark => "dark",
            ThemeVariant::Light => "light",
        };
        table.insert("variant".into(), Value::String(variant.into()));
        macro_rules! put {
            ($($name:ident),*) => {
                $(table.insert(stringify!($name).into(), Value::String(hex(self.$name)));)*
            };
        }
        required_colors!(put);
        if let Some(line) = self.tab_active_line {
            table.insert(OPTIONAL.into(), Value::String(hex(line)));
        }
        table.to_string()
    }

    /// Reads a theme file. Errors name the offending key.
    pub fn from_toml(text: &str) -> Result<Tokens, ThemeFileError> {
        let table: Table = text
            .parse()
            .map_err(|e: toml::de::Error| ThemeFileError::Syntax(e.message().to_owned()))?;
        const KNOWN: &[&str] = required_colors!(names);
        for key in table.keys() {
            if key != "variant" && key != OPTIONAL && !KNOWN.contains(&key.as_str()) {
                return Err(ThemeFileError::Unknown(key.clone()));
            }
        }
        let variant = match table.get("variant") {
            None => return Err(ThemeFileError::Missing("variant")),
            Some(v) => match v.as_str() {
                Some("dark") => ThemeVariant::Dark,
                Some("light") => ThemeVariant::Light,
                _ => return Err(ThemeFileError::BadVariant(v.to_string())),
            },
        };
        macro_rules! get {
            ($name:ident) => {
                parse_hex(
                    stringify!($name),
                    table
                        .get(stringify!($name))
                        .ok_or(ThemeFileError::Missing(stringify!($name)))?,
                )?
            };
        }
        let tab_active_line = match table.get(OPTIONAL) {
            Some(v) => Some(parse_hex(OPTIONAL, v)?),
            None => None,
        };
        macro_rules! build {
            ($($name:ident),*) => {
                Tokens {
                    $($name: get!($name),)*
                    tab_active_line,
                    // Series colors are not part of the file format yet; use the variant's palette.
                    #[cfg(feature = "series")]
                    series: match variant {
                        ThemeVariant::Dark => crate::SERIES_DARK,
                        ThemeVariant::Light => crate::SERIES_LIGHT,
                    },
                    variant,
                }
            };
        }
        Ok(required_colors!(build))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DARK, LIGHT};

    #[test]
    fn dark_and_light_round_trip() {
        for t in [DARK, LIGHT] {
            assert_eq!(Tokens::from_toml(&t.to_toml()).unwrap(), t);
        }
    }

    #[test]
    fn light_keeps_its_optional_outline_and_dark_omits_it() {
        assert!(LIGHT.to_toml().contains("tab_active_line = \"#AAB2BD\""));
        assert!(!DARK.to_toml().contains("tab_active_line"));
    }

    #[test]
    fn output_is_stable_and_hex_is_upper_case() {
        let text = DARK.to_toml();
        assert!(text.contains("ground = \"#101216\""));
        assert!(text.contains("variant = \"dark\""));
        assert_eq!(text, DARK.to_toml());
    }

    #[test]
    fn lower_case_hex_is_accepted() {
        let text = DARK.to_toml().replace("#101216", "#1a2b3c");
        assert_eq!(Tokens::from_toml(&text).unwrap().ground, [0x1a, 0x2b, 0x3c]);
    }

    fn with(replace_from: &str, to: &str) -> Result<Tokens, ThemeFileError> {
        Tokens::from_toml(&DARK.to_toml().replace(replace_from, to))
    }

    #[test]
    fn bad_hex_names_the_key_and_value() {
        for bad in ["#12345", "#1234567", "101216", "#GG1216", "#10 216"] {
            let err = with("\"#101216\"", &format!("\"{bad}\"")).unwrap_err();
            assert_eq!(
                err,
                ThemeFileError::BadColor {
                    key: "ground".into(),
                    value: format!("\"{bad}\"")
                }
            );
        }
        let err = with("\"#101216\"", "16").unwrap_err();
        assert!(matches!(err, ThemeFileError::BadColor { ref key, .. } if key == "ground"));
        assert_eq!(
            with("\"#101216\"", "\"#12345\"").unwrap_err().to_string(),
            "`ground` must be a color like \"#1A2B3C\", found \"#12345\""
        );
    }

    #[test]
    fn structural_errors_are_specific() {
        assert!(matches!(
            Tokens::from_toml("ground = "),
            Err(ThemeFileError::Syntax(_))
        ));
        assert_eq!(
            with("variant = \"dark\"\n", ""),
            Err(ThemeFileError::Missing("variant"))
        );
        assert_eq!(
            with("variant = \"dark\"", "variant = \"sepia\""),
            Err(ThemeFileError::BadVariant("\"sepia\"".into()))
        );
        assert_eq!(
            Tokens::from_toml("variant = \"dark\"\nground = \"#000000\"\n"),
            Err(ThemeFileError::Missing("bar"))
        );
        let extra = format!("{}colour = \"#000000\"\n", DARK.to_toml());
        assert_eq!(
            Tokens::from_toml(&extra),
            Err(ThemeFileError::Unknown("colour".into()))
        );
    }

    #[test]
    fn every_token_name_is_written() {
        const KNOWN: &[&str] = required_colors!(names);
        assert_eq!(KNOWN.len(), 34);
        let text = DARK.to_toml();
        for name in KNOWN {
            assert!(text.contains(&format!("{name} = ")), "{name}");
        }
    }
}
