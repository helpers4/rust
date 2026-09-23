// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgb;

/// Blends two colors channel by channel.
///
/// `t` is how far to go from `a` to `b`: `0.0` gives `a`, `1.0` gives `b`, `0.5` the midpoint. It
/// is clamped to `0.0..=1.0` (a `NaN` counts as `0.0`), and each channel is rounded to the nearest
/// integer. The blend happens in sRGB, without gamma correction, like CSS `color-mix(in srgb)`.
///
/// # Arguments
///
/// - `a` - The color at `t = 0`.
/// - `b` - The color at `t = 1`.
/// - `t` - How far from `a` towards `b`, `0.0..=1.0`.
///
/// # Returns
///
/// The blended color.
///
/// # Examples
///
/// ```
/// use helpers4::color::{mix, Rgb};
///
/// let red = Rgb::new(255, 0, 0);
/// let blue = Rgb::new(0, 0, 255);
/// assert_eq!(mix(red, blue, 0.5), Rgb::new(128, 0, 128));
/// assert_eq!(mix(red, blue, 0.0), red);
/// ```
#[must_use]
pub fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
    let blend = |x: u8, y: u8| to_channel(f64::from(x) * (1.0 - t) + f64::from(y) * t);
    Rgb::new(
        blend(a.r(), b.r()),
        blend(a.g(), b.g()),
        blend(a.b(), b.b()),
    )
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a blend of two u8s stays in 0..=255
fn to_channel(value: f64) -> u8 {
    value.round() as u8
}

#[cfg(test)]
#[path = "mix.test.rs"]
mod tests;

#[cfg(test)]
#[path = "mix.spec.rs"]
mod spec;
