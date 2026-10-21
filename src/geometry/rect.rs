// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// An axis-aligned rectangle: its top-left corner and its size. Width and height are expected
/// to be non-negative.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::Rect;
///
/// let r = Rect::new(10.0, 20.0, 100.0, 50.0);
/// assert_eq!((r.right(), r.bottom()), (110.0, 70.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    /// Left edge.
    pub x: f64,
    /// Top edge.
    pub y: f64,
    /// Width, `>= 0`.
    pub width: f64,
    /// Height, `>= 0`.
    pub height: f64,
}

impl Rect {
    /// Builds a rectangle from its top-left corner and its size.
    ///
    /// # Arguments
    ///
    /// - `x` - Left edge.
    /// - `y` - Top edge.
    /// - `width` - Width, `>= 0`.
    /// - `height` - Height, `>= 0`.
    ///
    /// # Returns
    ///
    /// The rectangle.
    #[must_use]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The right edge, `x + width`.
    ///
    /// # Returns
    ///
    /// The x coordinate of the right edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// assert_eq!(Rect::new(1.0, 0.0, 2.0, 0.0).right(), 3.0);
    /// ```
    #[must_use]
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// The bottom edge, `y + height`.
    ///
    /// # Returns
    ///
    /// The y coordinate of the bottom edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// assert_eq!(Rect::new(0.0, 1.0, 0.0, 2.0).bottom(), 3.0);
    /// ```
    #[must_use]
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

#[cfg(test)]
#[path = "rect.test.rs"]
mod tests;
