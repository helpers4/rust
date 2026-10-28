// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::FontStyle;

/// A face of a font family, with the properties CSS font matching compares. A variable face
/// covers a weight range.
///
/// # Examples
///
/// ```
/// use helpers4::font::{FontFace, FontStyle};
///
/// let bold = FontFace::new(700.0, FontStyle::Normal);
/// let variable = FontFace::variable(100.0, 900.0, FontStyle::Italic);
/// assert_eq!((bold.weight_min, bold.weight_max), (700.0, 700.0));
/// assert_eq!((variable.weight_min, variable.weight_max), (100.0, 900.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontFace {
    /// Lightest weight the face covers (CSS `font-weight`, 1–1000).
    pub weight_min: f64,
    /// Heaviest weight the face covers; equal to `weight_min` for a static face.
    pub weight_max: f64,
    /// The style of the face.
    pub style: FontStyle,
    /// CSS `font-stretch` as a percentage (`condensed` is 75, `normal` 100, `expanded` 125).
    pub stretch: f64,
}

impl FontFace {
    /// A static face of normal width.
    ///
    /// # Arguments
    ///
    /// - `weight` - The weight, 1–1000.
    /// - `style` - The style.
    ///
    /// # Returns
    ///
    /// The face.
    #[must_use]
    pub const fn new(weight: f64, style: FontStyle) -> Self {
        Self::variable(weight, weight, style)
    }

    /// A variable face of normal width covering a weight range.
    ///
    /// # Arguments
    ///
    /// - `weight_min` - The lightest weight.
    /// - `weight_max` - The heaviest weight.
    /// - `style` - The style.
    ///
    /// # Returns
    ///
    /// The face.
    #[must_use]
    pub const fn variable(weight_min: f64, weight_max: f64, style: FontStyle) -> Self {
        Self {
            weight_min,
            weight_max,
            style,
            stretch: 100.0,
        }
    }

    /// The same face with another width.
    ///
    /// # Arguments
    ///
    /// - `stretch` - The width as a percentage.
    ///
    /// # Returns
    ///
    /// The face.
    #[must_use]
    pub const fn with_stretch(mut self, stretch: f64) -> Self {
        self.stretch = stretch;
        self
    }
}

#[cfg(test)]
#[path = "font_face.test.rs"]
mod tests;
