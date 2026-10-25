//! Ratatui widgets for Graphite chrome, colored only from [`graphite_core::Tokens`].
//!
//! Graphite core lays chrome out in design pixels. This crate fixes a
//! cell-to-pixel ratio ([`CELL_W`] by [`CELL_H`]), measures text with
//! [`CellMetrics`], and draws the resulting layouts into a ratatui buffer.
//!
//! ```
//! use graphite_core::{TabText, Dot, DARK};
//! use graphite_tui::{tabs_bar_layout, TabsBar};
//! use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
//!
//! let tab = TabText {
//!     label: "build".into(),
//!     meta: None,
//!     dot: Dot::Working,
//!     attention: false,
//!     selected: true,
//! };
//! let layout = tabs_bar_layout(60, "Graphite", &[tab], "Ctrl K");
//! let area = Rect::new(0, 0, 60, 3);
//! let mut buf = Buffer::empty(area);
//! TabsBar::new(&layout, &DARK).render(area, &mut buf);
//! ```

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
