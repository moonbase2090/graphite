//! Round-trip the built-in themes through TOML and show the error for a bad color.
use graphite_core::{Tokens, DARK, LIGHT};

fn main() {
    for (name, tokens) in [("dark", DARK), ("light", LIGHT)] {
        let text = tokens.to_toml();
        let back = Tokens::from_toml(&text).expect("round trip");
        assert_eq!(back, tokens);
        println!("{name}: {} lines, round trip equal", text.lines().count());
    }
    let broken = DARK.to_toml().replace("#101216", "#10121");
    match Tokens::from_toml(&broken) {
        Err(error) => println!("bad hex: {error}"),
        Ok(_) => panic!("short hex must not parse"),
    }
}
