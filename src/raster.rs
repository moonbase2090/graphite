//! Backend-neutral CPU drawing into a row-major RGBA framebuffer.

use crate::Rect;

/// One straight-alpha sRGB pixel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Opacity, where `0` is transparent and `255` is opaque.
    pub a: u8,
}

impl Rgba {
    /// Creates a pixel from its straight-alpha sRGB channels.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Transparent black.
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

/// Why a pixel buffer could not be created.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferError {
    /// The requested dimensions exceed the addressable RGBA byte count.
    SizeOverflow,
    /// The allocator could not reserve the requested byte count.
    AllocationFailed,
}

/// A CPU framebuffer with row-major, straight-alpha RGBA8 pixels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelBuffer {
    width: usize,
    height: usize,
    rgba: Vec<u8>,
}

impl PixelBuffer {
    /// Creates a buffer filled with `clear`.
    pub fn new(width: usize, height: usize, clear: Rgba) -> Result<Self, FramebufferError> {
        let byte_len = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(FramebufferError::SizeOverflow)?;
        let mut rgba = Vec::new();
        rgba.try_reserve_exact(byte_len)
            .map_err(|_| FramebufferError::AllocationFailed)?;
        for _ in 0..width.saturating_mul(height) {
            rgba.extend_from_slice(&[clear.r, clear.g, clear.b, clear.a]);
        }
        Ok(Self {
            width,
            height,
            rgba,
        })
    }

    /// Buffer width in pixels.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Buffer height in pixels.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the framebuffer as row-major RGBA8 bytes.
    pub fn as_rgba(&self) -> &[u8] {
        &self.rgba
    }

    /// Returns the pixel at `(x, y)`, or `None` when it is outside the buffer.
    pub fn pixel(&self, x: usize, y: usize) -> Option<Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) * 4;
        Some(Rgba::new(
            self.rgba[index],
            self.rgba[index + 1],
            self.rgba[index + 2],
            self.rgba[index + 3],
        ))
    }

    /// Replaces every pixel with `color`.
    pub fn clear(&mut self, color: Rgba) {
        for pixel in self.rgba.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
        }
    }

    /// Composites `color` over the pixel at `(x, y)` using source-over alpha.
    /// Returns `false` if the pixel is outside the buffer.
    pub fn blend_pixel(&mut self, x: usize, y: usize, color: Rgba) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = (y * self.width + x) * 4;
        let destination = Rgba::new(
            self.rgba[index],
            self.rgba[index + 1],
            self.rgba[index + 2],
            self.rgba[index + 3],
        );
        let result = source_over(color, destination);
        self.rgba[index..index + 4].copy_from_slice(&[result.r, result.g, result.b, result.a]);
        true
    }

    /// Composites a filled rectangle over the buffer, clipping at its edges.
    /// Rectangles use Graphite's half-open pixel geometry.
    pub fn fill_rect(&mut self, rect: Rect, color: Rgba) {
        let x_start = rect.x.min(self.width);
        let y_start = rect.y.min(self.height);
        let x_end = rect.x.saturating_add(rect.w).min(self.width);
        let y_end = rect.y.saturating_add(rect.h).min(self.height);
        if x_start >= x_end || y_start >= y_end {
            return;
        }

        for y in y_start..y_end {
            for x in x_start..x_end {
                self.blend_pixel(x, y, color);
            }
        }
    }

    /// Composites a rectangular outline of `thickness` pixels, clipped at the buffer edges.
    pub fn stroke_rect(&mut self, rect: Rect, thickness: usize, color: Rgba) {
        if thickness == 0 || rect.w == 0 || rect.h == 0 {
            return;
        }

        let top = thickness.min(rect.h);
        let bottom = thickness.min(rect.h - top);
        let left = thickness.min(rect.w);
        let right = thickness.min(rect.w - left);

        self.fill_rect(Rect::new(rect.x, rect.y, rect.w, top), color);
        if bottom > 0 {
            self.fill_rect(
                Rect::new(
                    rect.x,
                    rect.y.saturating_add(rect.h - bottom),
                    rect.w,
                    bottom,
                ),
                color,
            );
        }
        let middle_height = rect.h - top - bottom;
        if middle_height > 0 {
            self.fill_rect(
                Rect::new(rect.x, rect.y.saturating_add(top), left, middle_height),
                color,
            );
            if right > 0 {
                self.fill_rect(
                    Rect::new(
                        rect.x.saturating_add(rect.w - right),
                        rect.y.saturating_add(top),
                        right,
                        middle_height,
                    ),
                    color,
                );
            }
        }
    }
}

fn source_over(source: Rgba, destination: Rgba) -> Rgba {
    let source_alpha = u32::from(source.a);
    let destination_alpha = u32::from(destination.a);
    let inverse_source_alpha = 255 - source_alpha;
    let alpha_numerator = source_alpha * 255 + destination_alpha * inverse_source_alpha;
    if alpha_numerator == 0 {
        return Rgba::TRANSPARENT;
    }

    let channel = |source_channel: u8, destination_channel: u8| {
        let numerator = u32::from(source_channel) * source_alpha * 255
            + u32::from(destination_channel) * destination_alpha * inverse_source_alpha;
        ((numerator + alpha_numerator / 2) / alpha_numerator) as u8
    };
    let alpha = ((alpha_numerator + 127) / 255) as u8;

    Rgba::new(
        channel(source.r, destination.r),
        channel(source.g, destination.g),
        channel(source.b, destination.b),
        alpha,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_has_stable_row_major_rgba_bytes() {
        let mut buffer = PixelBuffer::new(2, 1, Rgba::new(1, 2, 3, 4)).unwrap();
        assert_eq!(buffer.width(), 2);
        assert_eq!(buffer.height(), 1);
        assert_eq!(buffer.as_rgba(), &[1, 2, 3, 4, 1, 2, 3, 4]);

        buffer.clear(Rgba::new(9, 8, 7, 6));
        assert_eq!(buffer.pixel(1, 0), Some(Rgba::new(9, 8, 7, 6)));
    }

    #[test]
    fn constructor_rejects_dimensions_that_overflow_the_rgba_byte_count() {
        assert_eq!(
            PixelBuffer::new(usize::MAX, 1, Rgba::TRANSPARENT),
            Err(FramebufferError::SizeOverflow)
        );
    }

    #[test]
    fn rectangle_fill_clips_at_the_bottom_and_right_edges() {
        let mut buffer = PixelBuffer::new(3, 2, Rgba::TRANSPARENT).unwrap();
        buffer.fill_rect(
            Rect::new(1, 0, usize::MAX, usize::MAX),
            Rgba::new(20, 30, 40, 255),
        );

        assert_eq!(buffer.pixel(0, 0), Some(Rgba::TRANSPARENT));
        assert_eq!(buffer.pixel(1, 0), Some(Rgba::new(20, 30, 40, 255)));
        assert_eq!(buffer.pixel(2, 1), Some(Rgba::new(20, 30, 40, 255)));
        assert_eq!(buffer.pixel(3, 1), None);
    }

    #[test]
    fn source_over_blends_straight_alpha_and_preserves_transparent_source() {
        let mut buffer = PixelBuffer::new(2, 1, Rgba::new(0, 0, 255, 255)).unwrap();
        assert!(buffer.blend_pixel(0, 0, Rgba::new(255, 0, 0, 128)));
        assert_eq!(buffer.pixel(0, 0), Some(Rgba::new(128, 0, 127, 255)));

        assert!(buffer.blend_pixel(1, 0, Rgba::new(10, 20, 30, 0)));
        assert_eq!(buffer.pixel(1, 0), Some(Rgba::new(0, 0, 255, 255)));
        assert!(!buffer.blend_pixel(2, 0, Rgba::new(0, 0, 0, 255)));
    }

    #[test]
    fn source_over_on_transparent_retains_the_source_pixel() {
        let mut buffer = PixelBuffer::new(1, 1, Rgba::TRANSPARENT).unwrap();
        let color = Rgba::new(11, 22, 33, 127);
        assert!(buffer.blend_pixel(0, 0, color));
        assert_eq!(buffer.pixel(0, 0), Some(color));
    }

    #[test]
    fn source_over_composes_two_translucent_pixels() {
        let mut buffer = PixelBuffer::new(1, 1, Rgba::new(0, 0, 255, 128)).unwrap();
        assert!(buffer.blend_pixel(0, 0, Rgba::new(255, 0, 0, 128)));
        assert_eq!(buffer.pixel(0, 0), Some(Rgba::new(170, 0, 85, 192)));
    }

    #[test]
    fn rectangle_stroke_draws_non_overlapping_edges() {
        let mut buffer = PixelBuffer::new(4, 4, Rgba::TRANSPARENT).unwrap();
        buffer.stroke_rect(Rect::new(0, 0, 4, 4), 1, Rgba::new(5, 6, 7, 128));

        assert_eq!(buffer.pixel(0, 0), Some(Rgba::new(5, 6, 7, 128)));
        assert_eq!(buffer.pixel(1, 1), Some(Rgba::TRANSPARENT));
        assert_eq!(buffer.pixel(3, 2), Some(Rgba::new(5, 6, 7, 128)));
    }
}
