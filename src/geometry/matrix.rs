// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Point, Rect};

/// A 2D affine transformation (a 2×3 matrix), in the layout of CSS and SVG
/// `matrix(a, b, c, d, e, f)`:
///
/// ```text
/// x' = a·x + c·y + e
/// y' = b·x + d·y + f
/// ```
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{Matrix, Point};
///
/// // Scale by 2, then move by (10, 0): the last matrix of a product applies first.
/// let scale = Matrix::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0);
/// let shift = Matrix::new(1.0, 0.0, 0.0, 1.0, 10.0, 0.0);
/// let m = shift.multiply(scale);
/// assert_eq!(m.apply(Point::new(1.0, 1.0)), Point::new(12.0, 2.0));
/// assert_eq!(m.invert().map(|inv| inv.apply(Point::new(12.0, 2.0))), Some(Point::new(1.0, 1.0)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    /// Horizontal scale (with `c`, the x row of the linear part).
    pub a: f64,
    /// Vertical skew (with `d`, the y row of the linear part).
    pub b: f64,
    /// Horizontal skew.
    pub c: f64,
    /// Vertical scale.
    pub d: f64,
    /// Horizontal translation.
    pub e: f64,
    /// Vertical translation.
    pub f: f64,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Matrix {
    /// The transformation that changes nothing, also the [`Default`].
    pub const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    /// Builds a matrix from its six entries, in the order of CSS `matrix(a, b, c, d, e, f)`.
    ///
    /// # Arguments
    ///
    /// - `a`, `b`, `c`, `d` - The linear part (scale, rotation, skew).
    /// - `e`, `f` - The translation.
    ///
    /// # Returns
    ///
    /// The matrix.
    #[allow(clippy::many_single_char_names)] // the six entries are named a to f everywhere (CSS, SVG)
    #[must_use]
    pub const fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Self { a, b, c, d, e, f }
    }

    /// Multiplies `self` by `other`, in the mathematical order: `other` applies first, then
    /// `self`, as in a CSS transform list (`transform: self other`).
    ///
    /// # Arguments
    ///
    /// - `other` - The transformation applied first.
    ///
    /// # Returns
    ///
    /// The product, with `m.multiply(n).apply(p) == m.apply(n.apply(p))` (up to rounding).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::{Matrix, Point};
    ///
    /// let parent = Matrix::new(1.0, 0.0, 0.0, 1.0, 100.0, 50.0);
    /// let child = Matrix::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0);
    /// // A child layer's point, in the coordinates of the page
    /// assert_eq!(parent.multiply(child).apply(Point::new(1.0, 1.0)), Point::new(102.0, 52.0));
    /// ```
    #[must_use]
    pub fn multiply(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }

    /// The matrix that undoes `self`, e.g. to turn a pointer position into the coordinates of a
    /// transformed layer.
    ///
    /// A matrix that flattens the plane onto a line or a point (determinant `0`) cannot be
    /// undone. There is no tolerance: a nearly flat matrix is inverted, with very large entries,
    /// as long as they stay finite.
    ///
    /// # Returns
    ///
    /// The inverse, or `None` when `self` cannot be inverted (or has a non-finite entry).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Matrix;
    ///
    /// let shift = Matrix::new(1.0, 0.0, 0.0, 1.0, 10.0, 5.0);
    /// assert_eq!(shift.invert(), Some(Matrix::new(1.0, 0.0, 0.0, 1.0, -10.0, -5.0)));
    /// assert_eq!(Matrix::new(0.0, 0.0, 0.0, 0.0, 1.0, 1.0).invert(), None);
    /// ```
    #[must_use]
    pub fn invert(self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        // `+ 0.0` turns the -0.0 of a negated zero into 0.0, so inverses compare equal to the
        // matrices written by hand.
        let inverse = Self {
            a: self.d / det + 0.0,
            b: -self.b / det + 0.0,
            c: -self.c / det + 0.0,
            d: self.a / det + 0.0,
            e: (self.c * self.f - self.d * self.e) / det + 0.0,
            f: (self.b * self.e - self.a * self.f) / det + 0.0,
        };
        [
            inverse.a, inverse.b, inverse.c, inverse.d, inverse.e, inverse.f,
        ]
        .iter()
        .all(|v| v.is_finite())
        .then_some(inverse)
    }

    /// Transforms a point.
    ///
    /// # Arguments
    ///
    /// - `point` - The point to transform.
    ///
    /// # Returns
    ///
    /// `(a·x + c·y + e, b·x + d·y + f)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::{Matrix, Point};
    ///
    /// let m = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    /// assert_eq!(m.apply(Point::new(1.0, 1.0)), Point::new(9.0, 12.0));
    /// ```
    #[must_use]
    pub fn apply(self, point: Point) -> Point {
        Point {
            x: self.a * point.x + self.c * point.y + self.e,
            y: self.b * point.x + self.d * point.y + self.f,
        }
    }

    /// Transforms a rectangle and returns its bounding box: the smallest axis-aligned rectangle
    /// containing it. After a rotation or a skew, the box is larger than the rectangle.
    ///
    /// # Arguments
    ///
    /// - `rect` - The rectangle to transform.
    ///
    /// # Returns
    ///
    /// The bounding box of the four transformed corners, with a non-negative size.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::{Matrix, Rect};
    ///
    /// let flip = Matrix::new(-2.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    /// assert_eq!(flip.bounding_box(Rect::new(1.0, 0.0, 2.0, 3.0)), Rect::new(-6.0, 0.0, 4.0, 3.0));
    /// ```
    #[must_use]
    pub fn bounding_box(self, rect: Rect) -> Rect {
        let corners = [
            self.apply(Point::new(rect.x, rect.y)),
            self.apply(Point::new(rect.right(), rect.y)),
            self.apply(Point::new(rect.x, rect.bottom())),
            self.apply(Point::new(rect.right(), rect.bottom())),
        ];
        let (mut left, mut top) = (f64::INFINITY, f64::INFINITY);
        let (mut right, mut bottom) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in corners {
            left = left.min(p.x);
            top = top.min(p.y);
            right = right.max(p.x);
            bottom = bottom.max(p.y);
        }
        Rect::new(left, top, right - left, bottom - top)
    }
}

#[cfg(test)]
#[path = "matrix.test.rs"]
mod tests;

#[cfg(test)]
#[path = "matrix.spec.rs"]
mod spec;
