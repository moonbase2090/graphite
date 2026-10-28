//! Cell-space design tokens, independent of pixel-renderer geometry.

/// Spacing steps in terminal cells.
pub const CELL_SPACING: [u16; 6] = [0, 1, 2, 3, 4, 6];
/// Cells between the corner and a block title, before its surrounding spaces.
pub const BLOCK_TITLE_INSET: u16 = 1;

/// Supported terminal border styles. Double borders are deliberately absent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BorderStyle {
    /// Standard pane border.
    #[default]
    Rounded,
    /// Dense layouts.
    Plain,
    /// Focused blocks.
    Thick,
}

/// Border symbols in clockwise corner order, then horizontal and vertical edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderGlyphs {
    pub top_left: char,
    pub top_right: char,
    pub bottom_right: char,
    pub bottom_left: char,
    pub horizontal: char,
    pub vertical: char,
}

impl BorderStyle {
    /// Resolve once at startup. ASCII terminals use `+-|` for every style.
    #[must_use]
    pub const fn glyphs(self, ascii: bool) -> BorderGlyphs {
        let chars = if ascii {
            ['+', '+', '+', '+', '-', '|']
        } else {
            match self {
                Self::Rounded => ['╭', '╮', '╯', '╰', '─', '│'],
                Self::Plain => ['┌', '┐', '┘', '└', '─', '│'],
                Self::Thick => ['┏', '┓', '┛', '┗', '━', '┃'],
            }
        };
        BorderGlyphs {
            top_left: chars[0],
            top_right: chars[1],
            bottom_right: chars[2],
            bottom_left: chars[3],
            horizontal: chars[4],
            vertical: chars[5],
        }
    }
}

/// Sub-cell drawing family. Consumers probe their terminal/font once at startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubcellGlyphs {
    /// Unicode octants, when supported by the font.
    Octants,
    /// Two by four dots per cell.
    Braille,
    /// Half and quarter block shapes.
    HalfBlocks,
    /// Vertical eighths for gauge ends.
    EighthBlocks,
    /// Portable ASCII drawing, with `#` as the filled cell.
    Ascii,
}

/// Most capable to least capable. Select the first family supported by the host.
pub const SUBCELL_GLYPH_ORDER: [SubcellGlyphs; 5] = [
    SubcellGlyphs::Octants,
    SubcellGlyphs::Braille,
    SubcellGlyphs::HalfBlocks,
    SubcellGlyphs::EighthBlocks,
    SubcellGlyphs::Ascii,
];
/// Vertical eighth-block ramp, from empty to full.
pub const EIGHTH_BLOCKS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
/// Filled cell for ASCII-only drawing.
pub const ASCII_FILL: char = '#';

#[cfg(test)]
mod tests {
    use super::*;

    fn symbols(g: BorderGlyphs) -> String {
        [
            g.top_left,
            g.top_right,
            g.bottom_right,
            g.bottom_left,
            g.horizontal,
            g.vertical,
        ]
        .iter()
        .collect()
    }

    #[test]
    fn spacing_and_title_inset_match_cell_design() {
        assert_eq!(CELL_SPACING, [0, 1, 2, 3, 4, 6]);
        assert_eq!(BLOCK_TITLE_INSET, 1);
    }

    #[test]
    fn borders_have_exact_symbols_and_portable_fallback() {
        assert_eq!(BorderStyle::default(), BorderStyle::Rounded);
        assert_eq!(symbols(BorderStyle::Rounded.glyphs(false)), "╭╮╯╰─│");
        assert_eq!(symbols(BorderStyle::Plain.glyphs(false)), "┌┐┘└─│");
        assert_eq!(symbols(BorderStyle::Thick.glyphs(false)), "┏┓┛┗━┃");
        for style in [BorderStyle::Rounded, BorderStyle::Plain, BorderStyle::Thick] {
            assert_eq!(symbols(style.glyphs(true)), "++++-|");
        }
    }

    #[test]
    fn glyph_preference_and_fill_ramps_are_pinned() {
        assert_eq!(
            SUBCELL_GLYPH_ORDER,
            [
                SubcellGlyphs::Octants,
                SubcellGlyphs::Braille,
                SubcellGlyphs::HalfBlocks,
                SubcellGlyphs::EighthBlocks,
                SubcellGlyphs::Ascii
            ]
        );
        assert_eq!(EIGHTH_BLOCKS.iter().collect::<String>(), " ▁▂▃▄▅▆▇█");
        assert_eq!(ASCII_FILL, '#');
    }
}
