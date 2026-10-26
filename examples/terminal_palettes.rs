//! Regenerate the complete fallback reference with `cargo run --example terminal_palettes --features terminal-palette`.
use graphite_core::{terminal_palette, DARK, LIGHT};

fn main() {
    println!("| Theme | Token | RGB | xterm-256 | ANSI-16 |\n|---|---|---|---|---|");
    for (theme, tokens) in [("Dark", DARK), ("Light", LIGHT)] {
        for entry in terminal_palette(&tokens) {
            let [r, g, b] = entry.rgb;
            println!(
                "| {theme} | {} | #{r:02X}{g:02X}{b:02X} | {} | {:?} |",
                entry.name, entry.xterm256, entry.ansi16
            );
        }
    }
}
