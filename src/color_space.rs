use crate::Rgb;

// Matrices from https://bottosson.github.io/posts/oklab/ (2021 revision).
pub(crate) fn oklab(rgb: Rgb) -> [f64; 3] {
    let [r, g, b] = rgb.map(|channel| {
        let value = f64::from(channel) / 255.0;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    });
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}

#[cfg(feature = "motion")]
pub(crate) fn from_oklab([l, a, b]: [f64; 3]) -> Rgb {
    let ll = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    [
        4.0767416621 * ll - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * ll + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * ll - 0.7034186147 * m + 1.7076147010 * s,
    ]
    .map(|value| {
        let value = if value <= 0.0031308 {
            12.92 * value
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        };
        (value.clamp(0.0, 1.0) * 255.0).round() as u8
    })
}
