//! Draw and inspect a small RGBA frame through Graphite's public API.
use graphite_core::{PixelBuffer, Rect, Rgba};

fn main() {
    let mut frame = PixelBuffer::new(3, 2, Rgba::new(0, 0, 0, 0)).expect("small framebuffer");
    frame.fill_rect(Rect::new(0, 0, 3, 2), Rgba::new(0, 0, 255, 255));
    frame.fill_rect(Rect::new(1, 0, 2, 1), Rgba::new(255, 0, 0, 128));
    assert_eq!(frame.pixel(1, 0), Some(Rgba::new(128, 0, 127, 255)));
    assert_eq!(frame.pixel(0, 1), Some(Rgba::new(0, 0, 255, 255)));
    println!(
        "size={}x{} rgba={:?}",
        frame.width(),
        frame.height(),
        frame.as_rgba()
    );
}
