// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A width and a height.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::Size;
///
/// let frame = Size::new(390.0, 844.0);
/// assert_eq!((frame.width, frame.height), (390.0, 844.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    /// Width, `>= 0`.
    pub width: f64,
    /// Height, `>= 0`.
    pub height: f64,
}

impl Size {
    /// Builds a size.
    ///
    /// # Arguments
    ///
    /// - `width` - Width, `>= 0`.
    /// - `height` - Height, `>= 0`.
    ///
    /// # Returns
    ///
    /// The size.
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

#[cfg(test)]
#[path = "size.test.rs"]
mod tests;
