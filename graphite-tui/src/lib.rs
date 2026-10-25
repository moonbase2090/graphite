//! Ratatui widgets for Graphite chrome, colored only from [`graphite_core::Tokens`].
//!
//! Graphite core lays chrome out in design pixels. This crate fixes a
//! cell-to-pixel ratio ([`CELL_W`] by [`CELL_H`]), measures text with
//! [`CellMetrics`], and draws the resulting layouts into a ratatui buffer.

mod cells;
mod layout;
mod legend;
mod pane;
mod picker;
mod side_rail;
mod sidebar;
mod style;
mod tabs_bar;

pub use cells::{CellMetrics, CELL_H, CELL_W, SCALE_MILLI};
pub use layout::{legend_layout, picker_layout, tabs_bar_layout};
pub use legend::Legend;
pub use pane::PaneChrome;
pub use picker::Picker;
pub use side_rail::SideRail;
pub use sidebar::Sidebar;
pub use tabs_bar::TabsBar;
