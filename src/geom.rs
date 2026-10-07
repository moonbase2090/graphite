//! Renderer-independent geometry primitives.

/// A half-open pixel rectangle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
    /// Horizontal origin.
    pub x: usize,
    /// Vertical origin.
    pub y: usize,
    /// Width.
    pub w: usize,
    /// Height.
    pub h: usize,
}

impl Rect {
    /// Creates a rectangle at `(x, y)` with the given dimensions.
    pub const fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    /// Returns whether `(x, y)` is inside this half-open rectangle.
    pub const fn contains(self, x: usize, y: usize) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    /// Returns the exclusive right edge.
    pub const fn right(self) -> usize {
        self.x + self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_includes_top_left_and_excludes_bottom_right_edges() {
        let rect = Rect::new(10, 20, 30, 40);
        assert!(rect.contains(10, 20));
        assert!(rect.contains(39, 59));
        assert!(!rect.contains(40, 59));
        assert!(!rect.contains(39, 60));
        assert!(!rect.contains(9, 20));
        assert!(!rect.contains(10, 19));
    }

    #[test]
    fn zero_sized_rect_contains_nothing() {
        assert!(!Rect::new(10, 20, 0, 0).contains(10, 20));
        assert!(!Rect::new(10, 20, 0, 4).contains(10, 20));
        assert!(!Rect::new(10, 20, 4, 0).contains(10, 20));
    }
}
