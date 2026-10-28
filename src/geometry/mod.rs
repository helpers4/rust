// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! 2D geometry in `f64`: [`Point`], [`Rect`] and the affine [`Matrix`], in the layout of CSS and
//! SVG `matrix(a b c d e f)`.
//!
//! Enough to map between coordinate spaces: build and multiply transforms, invert one to bring a
//! point back, take the bounding box of a transformed rectangle (hit areas, gradient bounds), place
//! content in a container like CSS `object-fit` ([`fit`]), simplify a polyline, and work with SVG
//! path data: parse it ([`parse_path`]), transform it by a matrix ([`transform_path`]) and write
//! rectangles and ellipses as paths ([`rounded_rect_path`], [`ellipse_path`]).

mod _arc_to_cubics;
mod _normalize_path;
mod ellipse_path;
mod fit;
mod format_path;
mod matrix;
mod parse_path;
mod path_command;
mod path_error;
mod point;
mod rect;
mod rounded_rect_path;
mod simplify_polyline;
mod size;
mod transform_path;

pub use ellipse_path::ellipse_path;
pub use fit::FitAlign;
pub use fit::FitMode;
pub use fit::fit;
pub use format_path::format_path;
pub use matrix::Matrix;
pub use parse_path::parse_path;
pub use path_command::PathCommand;
pub use path_error::ParsePathError;
pub use point::Point;
pub use rect::Rect;
pub use rounded_rect_path::rounded_rect_path;
pub use simplify_polyline::simplify_polyline;
pub use size::Size;
pub use transform_path::transform_path;
