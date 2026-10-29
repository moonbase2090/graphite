//! The only place token bytes become ratatui colors.

use graphite_core::{text_ink, Dot, Rgb, Tokens};
use ratatui::style::{Color, Style};

pub(crate) fn color(c: Rgb) -> Color {
    Color::Rgb(c[0], c[1], c[2])
}

pub(crate) fn style(fg: Rgb, bg: Rgb) -> Style {
    Style::new().fg(color(fg)).bg(color(bg))
}

pub(crate) fn text_style(preferred: Rgb, bg: Rgb) -> Style {
    style(text_ink(preferred, bg), bg)
}

pub(crate) fn dot_color(t: &Tokens, dot: Dot) -> Rgb {
    match dot {
        Dot::Attention => t.attention,
        Dot::Working => t.working,
        Dot::Unseen => t.unseen,
        Dot::Idle => t.idle,
    }
}
