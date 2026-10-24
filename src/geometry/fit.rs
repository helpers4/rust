// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Matrix, Size};

/// How [`fit`] sizes content in a container, as CSS `object-fit`.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{fit, FitAlign, FitMode, Size};
///
/// let frame = Size::new(200.0, 100.0);
/// let viewport = Size::new(100.0, 100.0);
/// let whole = fit(frame, viewport, FitMode::Contain, FitAlign::CENTER);
/// let filled = fit(frame, viewport, FitMode::Cover, FitAlign::CENTER);
/// assert_eq!((whole.a, filled.a), (0.5, 1.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FitMode {
    /// As large as possible while staying entirely inside (bands remain on two sides). The default.
    #[default]
    Contain,
    /// As small as possible while covering the whole container (it overflows on two sides).
    Cover,
    /// Stretched to the container on each axis, proportions not kept.
    Fill,
    /// Not scaled (CSS `object-fit: none`).
    Original,
    /// Like [`Contain`](Self::Contain), but never enlarged.
    ScaleDown,
}

/// Where [`fit`] places content in the space left over (or overflowing), as CSS
/// `object-position`: `0.0` is left or top, `0.5` centered, `1.0` right or bottom.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::FitAlign;
///
/// assert_eq!(FitAlign::default(), FitAlign::CENTER);
/// assert_eq!(FitAlign::new(0.0, 0.0).x, 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FitAlign {
    /// Horizontal position.
    pub x: f64,
    /// Vertical position.
    pub y: f64,
}

impl FitAlign {
    /// Centered on both axes.
    pub const CENTER: Self = Self { x: 0.5, y: 0.5 };

    /// Builds an alignment.
    ///
    /// # Arguments
    ///
    /// - `x` - Horizontal position, `0.0` (left) to `1.0` (right).
    /// - `y` - Vertical position, `0.0` (top) to `1.0` (bottom).
    ///
    /// # Returns
    ///
    /// The alignment.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for FitAlign {
    fn default() -> Self {
        Self::CENTER
    }
}

/// Places content in a container like CSS `object-fit` and `object-position`, e.g. to show a
/// design frame in a viewport of another size.
///
/// The result is the matrix from content coordinates to container coordinates: `a` and `d` are
/// the scales, `e` and `f` the position of the content's top-left corner. Map points with
/// [`Matrix::apply`], combine it with [`Matrix::multiply`], or write it with
/// [`Matrix::to_css`]. Sizes are expected to be positive.
///
/// # Arguments
///
/// - `content` - The size of the content.
/// - `container` - The size of the container.
/// - `mode` - How to size the content.
/// - `align` - Where to place it.
///
/// # Returns
///
/// The content-to-container matrix.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{fit, FitAlign, FitMode, Matrix, Size};
///
/// let m = fit(Size::new(200.0, 100.0), Size::new(100.0, 100.0), FitMode::Contain, FitAlign::CENTER);
/// assert_eq!(m, Matrix::new(0.5, 0.0, 0.0, 0.5, 0.0, 25.0));
///
/// let left = fit(Size::new(200.0, 100.0), Size::new(100.0, 100.0), FitMode::Cover, FitAlign::new(0.0, 0.5));
/// assert_eq!(left, Matrix::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)); // cropped on the right
/// ```
#[must_use]
pub fn fit(content: Size, container: Size, mode: FitMode, align: FitAlign) -> Matrix {
    let scale_x = container.width / content.width;
    let scale_y = container.height / content.height;
    let (sx, sy) = match mode {
        FitMode::Contain => {
            let s = scale_x.min(scale_y);
            (s, s)
        }
        FitMode::Cover => {
            let s = scale_x.max(scale_y);
            (s, s)
        }
        FitMode::Fill => (scale_x, scale_y),
        FitMode::Original => (1.0, 1.0),
        FitMode::ScaleDown => {
            let s = scale_x.min(scale_y).min(1.0);
            (s, s)
        }
    };
    // `+ 0.0` turns the -0.0 of an overflow aligned at 0 into 0.0.
    Matrix::new(
        sx,
        0.0,
        0.0,
        sy,
        (container.width - content.width * sx) * align.x + 0.0,
        (container.height - content.height * sy) * align.y + 0.0,
    )
}

#[cfg(test)]
#[path = "fit.test.rs"]
mod tests;

#[cfg(test)]
#[path = "fit.spec.rs"]
mod spec;
