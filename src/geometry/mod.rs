// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! 2D geometry in `f64`: [`Point`], [`Rect`] and the affine [`Matrix`], in the layout of CSS and
//! SVG `matrix(a b c d e f)`.
//!
//! Enough to map between coordinate spaces: build and multiply transforms, invert one to bring a
//! point back, take the bounding box of a transformed rectangle (hit areas, gradient bounds), place
//! content in a container like CSS `object-fit` ([`fit`]), and simplify a polyline.

mod fit;
mod matrix;
mod point;
mod rect;
mod simplify_polyline;
mod size;

pub use fit::FitAlign;
pub use fit::FitMode;
pub use fit::fit;
pub use matrix::Matrix;
pub use point::Point;
pub use rect::Rect;
pub use simplify_polyline::simplify_polyline;
pub use size::Size;
