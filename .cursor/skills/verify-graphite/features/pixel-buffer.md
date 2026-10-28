# Pixel buffer

The opt-in pixel buffer composites straight-alpha RGBA8 pixels into a CPU frame. A caller can fill and stroke clipped rectangles and read the resulting row-major bytes for its own backend transport.

## Sub-features

- `pixel-buffer-blend` composites a half-alpha red rectangle over an opaque blue fill.
- `pixel-buffer-stroke` draws a green outline without filling the center pixel.
- `pixel-buffer-bytes` exposes exactly four bytes per pixel in row-major order.

## How to get to it (user POV)

- A host creates a frame with its desired clear color.
- The host fills and strokes rectangles, then passes the resulting RGBA bytes to its own renderer or transport.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.
- The verification driver enables the opt-in `pixel-buffer` feature.

Run `.cursor/skills/verify-graphite/scripts/drive.sh pixel-buffer`. The evidence file `target/verify-graphite/pixel-buffer/drive.txt` contains:

```text
feature pixel-buffer
dimensions 3x3
blend Rgba { r: 128, g: 0, b: 127, a: 255 }
top-center Rgba { r: 0, g: 255, b: 0, a: 255 }
bottom-center Rgba { r: 0, g: 0, b: 255, a: 255 }
clipped-bottom-right Rgba { r: 255, g: 255, b: 0, a: 255 }
rgba-bytes [0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 255, 0, 255]
```

The driver calls `PixelBuffer::new`, `fill_rect`, `stroke_rect`, `pixel`, and `as_rgba` through the public API. It checks the results against literal colors and byte count.

## Gotchas

- `fill_rect` and `stroke_rect` composite source-over; `clear` replaces existing pixels.
- Rectangles use half-open `Rect` geometry and are clipped to the buffer dimensions.
- RGBA bytes use straight alpha and do not include any Kitty or other transport framing.
