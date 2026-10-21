// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! 2D geometry in `f64`: [`Point`], [`Rect`] and the affine [`Matrix`], in the layout of CSS and
//! SVG `matrix(a b c d e f)`.
//!
//! Enough to map between coordinate spaces: multiply transforms, invert one to bring a point
//! back, and take the bounding box of a transformed rectangle (hit areas, gradient bounds).

mod matrix;
mod point;
mod rect;

pub use matrix::Matrix;
pub use point::Point;
pub use rect::Rect;
