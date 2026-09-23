// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgb;

/// A color as hue, saturation and lightness.
///
/// The hue is in degrees, `0.0..360.0`; saturation and lightness are fractions from `0.0` to `1.0`.
/// [`Hsl::new`] brings any input into those ranges. Get one from [`Rgb::to_hsl`] and go back with
/// [`Hsl::to_rgb`].
///
/// # Examples
///
/// ```
/// use helpers4::color::{Hsl, Rgb};
///
/// let teal = Hsl::new(180.0, 1.0, 0.25);
/// assert_eq!(teal.to_rgb(), Rgb::new(0, 128, 128));
/// assert_eq!(Hsl::new(-90.0, 2.0, -1.0), Hsl::new(270.0, 1.0, 0.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    h: f64,
    s: f64,
    l: f64,
}

impl Hsl {
    /// Builds a color, normalizing the values: the hue wraps around 360 degrees, saturation and
    /// lightness are clamped to `0.0..=1.0`, and a `NaN` counts as `0.0`.
    ///
    /// # Arguments
    ///
    /// - `h` - The hue in degrees.
    /// - `s` - The saturation, `0.0` (grey) to `1.0` (vivid).
    /// - `l` - The lightness, `0.0` (black) to `1.0` (white).
    ///
    /// # Returns
    ///
    /// The normalized color.
    #[must_use]
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        let clean = |value: f64| if value.is_nan() { 0.0 } else { value };
        Self {
            h: clean(h).rem_euclid(360.0),
            s: clean(s).clamp(0.0, 1.0),
            l: clean(l).clamp(0.0, 1.0),
        }
    }

    /// The hue in degrees.
    ///
    /// # Returns
    ///
    /// A value in `0.0..360.0`.
    #[must_use]
    pub fn h(self) -> f64 {
        self.h
    }

    /// The saturation.
    ///
    /// # Returns
    ///
    /// A value from `0.0` to `1.0`.
    #[must_use]
    pub fn s(self) -> f64 {
        self.s
    }

    /// The lightness.
    ///
    /// # Returns
    ///
    /// A value from `0.0` to `1.0`.
    #[must_use]
    pub fn l(self) -> f64 {
        self.l
    }

    /// The nearest 8-bit sRGB color.
    ///
    /// # Returns
    ///
    /// The [`Rgb`] color, each channel rounded to the nearest integer.
    #[must_use]
    pub fn to_rgb(self) -> Rgb {
        let chroma = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let sector = self.h / 60.0;
        let second = chroma * (1.0 - (sector % 2.0 - 1.0).abs());
        let (r, g, b) = if sector < 1.0 {
            (chroma, second, 0.0)
        } else if sector < 2.0 {
            (second, chroma, 0.0)
        } else if sector < 3.0 {
            (0.0, chroma, second)
        } else if sector < 4.0 {
            (0.0, second, chroma)
        } else if sector < 5.0 {
            (second, 0.0, chroma)
        } else {
            (chroma, 0.0, second)
        };
        let offset = self.l - chroma / 2.0;
        Rgb::new(
            channel(r + offset),
            channel(g + offset),
            channel(b + offset),
        )
    }
}

/// A `0.0..=1.0` intensity as an 8-bit channel.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to 0..=255 first
fn channel(intensity: f64) -> u8 {
    (intensity * 255.0).round().clamp(0.0, 255.0) as u8
}

#[cfg(test)]
#[path = "hsl.test.rs"]
mod tests;
