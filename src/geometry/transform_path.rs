// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_normalize_path::normalize_path;
use super::{Matrix, ParsePathError, Point, format_path, parse_path};

/// Transforms SVG path data by a matrix. The result is written with absolute `M`, `L`, `C`, `Q`
/// and `Z` only: relative commands are made absolute, `H`/`V`/`S`/`T` are expanded, and arcs
/// become cubic curves (so any matrix works, skews included). Moving a path is the common case.
///
/// # Arguments
///
/// - `d` - The path data.
/// - `matrix` - The matrix, in the CSS/SVG layout.
/// - `precision` - Decimals kept per number; `6` is a good default.
///
/// # Returns
///
/// The transformed path data.
///
/// # Errors
///
/// A [`ParsePathError`] if `d` is not valid path data.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{transform_path, Matrix};
///
/// let moved = transform_path("M0 0 h10 v10 z", Matrix::translate(5.0, 5.0), 6)?;
/// assert_eq!(moved, "M5 5 L15 5 L15 15 Z");
/// # Ok::<(), helpers4::geometry::ParsePathError>(())
/// ```
pub fn transform_path(d: &str, matrix: Matrix, precision: usize) -> Result<String, ParsePathError> {
    let mut commands = normalize_path(&parse_path(d)?);
    for command in &mut commands {
        for pair in command.values.chunks_exact_mut(2) {
            let moved = matrix.apply(Point::new(pair[0], pair[1]));
            pair[0] = moved.x;
            pair[1] = moved.y;
        }
    }
    Ok(format_path(&commands, precision))
}

#[cfg(test)]
#[path = "transform_path.test.rs"]
mod tests;

#[cfg(test)]
#[path = "transform_path.spec.rs"]
mod spec;
