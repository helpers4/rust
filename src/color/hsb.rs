// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgb;

/// A color as hue, saturation and brightness (also called HSV), the model of design tool color
/// pickers. Unlike [`Hsl`](super::Hsl) lightness, brightness `1.0` is the pure color, not white.
///
/// The hue is in degrees, `0.0..360.0`; saturation and brightness are fractions from `0.0` to
/// `1.0`. [`Hsb::new`] brings any input into those ranges. Get one from [`Rgb::to_hsb`] and go
/// back with [`Hsb::to_rgb`].
///
/// # Examples
///
/// ```
/// use helpers4::color::{Hsb, Rgb};
///
/// assert_eq!(Hsb::new(0.0, 1.0, 1.0).to_rgb(), Rgb::new(255, 0, 0));
/// assert_eq!(Rgb::new(0, 128, 128).to_hsb(), Hsb::new(180.0, 1.0, 128.0 / 255.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsb {
    h: f64,
    s: f64,
    b: f64,
}

impl Hsb {
    /// Builds a color, normalizing the values: the hue wraps around 360 degrees, saturation and
    /// brightness are clamped to `0.0..=1.0`, and a `NaN` counts as `0.0`.
    ///
    /// # Arguments
    ///
    /// - `h` - The hue in degrees.
    /// - `s` - The saturation, `0.0` (grey) to `1.0` (vivid).
    /// - `b` - The brightness, `0.0` (black) to `1.0` (the pure color).
    ///
    /// # Returns
    ///
    /// The normalized color.
    #[must_use]
    pub fn new(h: f64, s: f64, b: f64) -> Self {
        let clean = |value: f64| if value.is_nan() { 0.0 } else { value };
        Self {
            h: clean(h).rem_euclid(360.0),
            s: clean(s).clamp(0.0, 1.0),
            b: clean(b).clamp(0.0, 1.0),
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

    /// The brightness.
    ///
    /// # Returns
    ///
    /// A value from `0.0` to `1.0`.
    #[must_use]
    pub fn b(self) -> f64 {
        self.b
    }

    /// The nearest 8-bit sRGB color.
    ///
    /// # Returns
    ///
    /// The [`Rgb`] color, each channel rounded to the nearest integer.
    #[must_use]
    pub fn to_rgb(self) -> Rgb {
        let chroma = self.b * self.s;
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
        let offset = self.b - chroma;
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
#[path = "hsb.test.rs"]
mod tests;
