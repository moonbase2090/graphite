//! Print the resolved cell-space design tokens through the public API.
use graphite_core::{
    BorderStyle, ASCII_FILL, BLOCK_TITLE_INSET, CELL_SPACING, EIGHTH_BLOCKS, SUBCELL_GLYPH_ORDER,
};
fn main() {
    println!("spacing={CELL_SPACING:?} title-inset={BLOCK_TITLE_INSET}");
    for style in [BorderStyle::Rounded, BorderStyle::Plain, BorderStyle::Thick] {
        for ascii in [false, true] {
            let g = style.glyphs(ascii);
            println!(
                "{style:?} ascii={ascii}: {}{}{}{}{}{}",
                g.top_left, g.top_right, g.bottom_right, g.bottom_left, g.horizontal, g.vertical
            );
        }
    }
    println!("subcell={SUBCELL_GLYPH_ORDER:?}");
    println!(
        "eighths={} ascii-fill={ASCII_FILL}",
        EIGHTH_BLOCKS.iter().collect::<String>()
    );
}
