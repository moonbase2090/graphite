//! Core layouts computed with cell metrics so widgets land on whole cells.

use crate::cells::{CellMetrics, CELL_H, CELL_W, SCALE_MILLI};
use graphite_core::{
    bar_layout, legend_keys, theme_picker_layout, BarLayout, LegendPaint, LegendRun, TabText,
    ThemePickerLayout,
};

/// Lays out the tabs bar across `cols` columns.
pub fn tabs_bar_layout(cols: u16, space: &str, tabs: &[TabText], chord: &str) -> BarLayout {
    bar_layout(
        &CellMetrics,
        SCALE_MILLI,
        usize::from(cols) * CELL_W,
        0,
        space,
        tabs,
        chord,
    )
}

/// Packs legend runs into `cols` columns and `rows` rows ending at the bottom of a `height`-row area.
pub fn legend_layout(cols: u16, height: u16, rows: usize, runs: &[LegendRun]) -> LegendPaint {
    legend_keys(
        &CellMetrics,
        SCALE_MILLI,
        runs,
        usize::from(cols) * CELL_W,
        usize::from(height) * CELL_H,
        rows,
    )
}

/// Lays out the theme picker in a `cols` by `rows` window, if it fits.
pub fn picker_layout(
    cols: u16,
    rows: u16,
    row_count: usize,
    scroll: usize,
) -> Option<ThemePickerLayout> {
    theme_picker_layout(
        SCALE_MILLI,
        row_count,
        scroll,
        usize::from(cols) * CELL_W,
        usize::from(rows) * CELL_H,
    )
}
