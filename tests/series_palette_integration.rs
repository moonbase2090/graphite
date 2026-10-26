#![cfg(all(feature = "series", feature = "terminal-palette"))]
use graphite_core::{terminal_palette, Ansi16, DARK, LIGHT};
#[test]
fn terminal_fallback_covers_every_series_token() {
    for (tokens, expected) in [
        (DARK, [75, 42, 214, 141, 209, 44]),
        (LIGHT, [26, 29, 94, 98, 160, 30]),
    ] {
        let table = terminal_palette(&tokens);
        let entries: Vec<_> = table
            .iter()
            .filter(|entry| entry.name.starts_with("series-"))
            .collect();
        assert_eq!(entries.len(), 6);
        assert_eq!(
            entries.iter().map(|entry| entry.name).collect::<Vec<_>>(),
            ["series-1", "series-2", "series-3", "series-4", "series-5", "series-6"]
        );
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.xterm256)
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            entries.iter().map(|entry| entry.rgb).collect::<Vec<_>>(),
            tokens.series
        );
        assert_eq!(
            entries.iter().map(|entry| entry.ansi16).collect::<Vec<_>>(),
            [
                Ansi16::Blue,
                Ansi16::Green,
                Ansi16::Yellow,
                Ansi16::Magenta,
                Ansi16::Red,
                Ansi16::Cyan
            ]
        );
    }
}
