// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A point in 2D space.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::Point;
///
/// let p = Point::new(3.0, 4.0);
/// assert_eq!((p.x, p.y), (3.0, 4.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f64,
    /// Vertical coordinate.
    pub y: f64,
}

impl Point {
    /// Builds a point from its coordinates.
    ///
    /// # Arguments
    ///
    /// - `x` - Horizontal coordinate.
    /// - `y` - Vertical coordinate.
    ///
    /// # Returns
    ///
    /// The point.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
#[path = "point.test.rs"]
mod tests;
