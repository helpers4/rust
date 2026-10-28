// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{PathCommand, Rect, format_path};

/// The path of the ellipse that fills a box, as SVG path data: two half-turn arcs. A square box
/// gives a circle.
///
/// # Arguments
///
/// - `bounds` - The bounding box of the ellipse.
///
/// # Returns
///
/// The path data, closed.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{ellipse_path, Rect};
///
/// assert_eq!(
///     ellipse_path(Rect::new(0.0, 0.0, 10.0, 6.0)),
///     "M0 3 A5 3 0 0 1 10 3 A5 3 0 0 1 0 3 Z"
/// );
/// ```
#[must_use]
pub fn ellipse_path(bounds: Rect) -> String {
    let (rx, ry) = (bounds.width / 2.0, bounds.height / 2.0);
    let cy = bounds.y + ry;
    format_path(
        &[
            PathCommand::new('M', vec![bounds.x, cy]),
            PathCommand::new('A', vec![rx, ry, 0.0, 0.0, 1.0, bounds.right(), cy]),
            PathCommand::new('A', vec![rx, ry, 0.0, 0.0, 1.0, bounds.x, cy]),
            PathCommand::new('Z', Vec::new()),
        ],
        6,
    )
}

#[cfg(test)]
#[path = "ellipse_path.test.rs"]
mod tests;
