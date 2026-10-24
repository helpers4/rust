// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Point;

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

    /// The smallest rectangle that contains both `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `other` - The other rectangle.
    ///
    /// # Returns
    ///
    /// The bounding rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// let b = Rect::new(20.0, 5.0, 10.0, 10.0);
    /// assert_eq!(a.union(&b), Rect::new(0.0, 0.0, 30.0, 15.0));
    /// ```
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let left = self.x.min(other.x);
        let top = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        Self::new(left, top, right - left, bottom - top)
    }

    /// The smallest rectangle that contains all of `rects`, e.g. the bounds of a group of layers.
    ///
    /// # Arguments
    ///
    /// - `rects` - The rectangles.
    ///
    /// # Returns
    ///
    /// The bounding rectangle, or `None` for no rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// let group = [Rect::new(0.0, 0.0, 10.0, 10.0), Rect::new(20.0, 5.0, 10.0, 10.0)];
    /// assert_eq!(Rect::bounding(&group), Some(Rect::new(0.0, 0.0, 30.0, 15.0)));
    /// assert_eq!(Rect::bounding(&[]), None);
    /// ```
    #[must_use]
    pub fn bounding(rects: &[Self]) -> Option<Self> {
        rects.iter().copied().reduce(|a, b| a.union(&b))
    }

    /// The area `self` and `other` have in common.
    ///
    /// Rectangles that only touch (along an edge or at a corner) have no area in common.
    ///
    /// # Arguments
    ///
    /// - `other` - The other rectangle.
    ///
    /// # Returns
    ///
    /// The overlapping rectangle, or `None` when they do not overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// assert_eq!(a.intersection(&Rect::new(5.0, 5.0, 10.0, 10.0)), Some(Rect::new(5.0, 5.0, 5.0, 5.0)));
    /// assert_eq!(a.intersection(&Rect::new(10.0, 0.0, 10.0, 10.0)), None); // only touching
    /// ```
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = self.right().min(other.right()) - x;
        let height = self.bottom().min(other.bottom()) - y;
        (width > 0.0 && height > 0.0).then(|| Self::new(x, y, width, height))
    }

    /// Whether `self` and `other` have an area in common: exactly when
    /// [`intersection`](Self::intersection) returns a rectangle (touching rectangles and empty
    /// ones do not).
    ///
    /// # Arguments
    ///
    /// - `other` - The other rectangle.
    ///
    /// # Returns
    ///
    /// `true` if they overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// assert!(a.intersects(&Rect::new(5.0, 5.0, 10.0, 10.0)));
    /// assert!(!a.intersects(&Rect::new(10.0, 0.0, 10.0, 10.0)));
    /// ```
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    /// Whether a point is inside the rectangle, for hit-testing.
    ///
    /// The left and top edges are inside, the right and bottom ones are not, so a point on the
    /// edge between two adjacent rectangles hits exactly one of them.
    ///
    /// # Arguments
    ///
    /// - `point` - The point to test.
    ///
    /// # Returns
    ///
    /// `true` if the point is inside.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::{Point, Rect};
    ///
    /// let r = Rect::new(0.0, 0.0, 10.0, 10.0);
    /// assert!(r.contains_point(Point::new(0.0, 5.0)));
    /// assert!(!r.contains_point(Point::new(10.0, 5.0)));
    /// ```
    #[must_use]
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.x && point.x < self.right() && point.y >= self.y && point.y < self.bottom()
    }

    /// Whether `other` is inside the rectangle: it does not stick out on any side, and may touch
    /// any edge.
    ///
    /// # Arguments
    ///
    /// - `other` - The rectangle to test.
    ///
    /// # Returns
    ///
    /// `true` if `other` is entirely inside (or equal to) `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::geometry::Rect;
    ///
    /// let screen = Rect::new(0.0, 0.0, 100.0, 100.0);
    /// assert!(screen.contains_rect(&Rect::new(10.0, 10.0, 90.0, 90.0)));
    /// assert!(!screen.contains_rect(&Rect::new(50.0, 50.0, 100.0, 10.0)));
    /// ```
    #[must_use]
    pub fn contains_rect(&self, other: &Self) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
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

#[cfg(test)]
#[path = "rect.spec.rs"]
mod spec;
