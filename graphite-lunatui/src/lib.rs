// Copyright (c) 2026 Moonbase 2090 LLC
// SPDX-License-Identifier: MIT

//! Lunatui widgets for Graphite chrome, colored only from [`graphite_core::Tokens`].
//!
//! Graphite core lays chrome out in design pixels. This crate fixes a
//! cell-to-pixel ratio ([`CELL_W`] by [`CELL_H`]), measures text with
//! [`CellMetrics`], and draws the resulting layouts as lunatui widgets. It is
//! the lunatui counterpart of the `graphite-tui` Ratatui crate and renders
//! the same layouts cell for cell.
//!
//! ```
//! use graphite_core::{Dot, TabText, DARK};
//! use graphite_lunatui::{tabs_bar_layout, TabsBar};
//! use lunatui_core::{ClusterTable, Grid, Rect};
//! use lunatui_theme::{Theme, Variant};
//! use lunatui_widgets::{RenderCx, Widget};
//!
//! let tab = TabText {
//!     label: "build".into(),
//!     meta: None,
//!     dot: Dot::Working,
//!     attention: false,
//!     selected: true,
//! };
//! let layout = tabs_bar_layout(60, "Graphite", &[tab], "Ctrl K");
//! let theme = Theme::builtin(Variant::Dark);
//! let mut grid = Grid::new(60, 3);
//! let mut clusters = ClusterTable::new();
//! let mut cx = RenderCx::new(&mut grid, &mut clusters, &theme);
//! TabsBar::new(&layout, &DARK).render(Rect::new(0, 0, 60, 3), &mut cx);
//! ```

mod cells;
mod chrome;
mod layout;
mod pane;
mod side_rail;
mod style;
mod tabs_bar;

pub use cells::{CellMetrics, CELL_H, CELL_W, SCALE_MILLI};
pub use chrome::{Badge, Chip, StatusDot};
pub use layout::{legend_layout, picker_layout, tabs_bar_layout};
pub use pane::PaneChrome;
pub use side_rail::SideRail;
pub use tabs_bar::TabsBar;
