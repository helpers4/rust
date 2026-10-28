// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{PathCommand, Rect, format_path};

/// The path of a rectangle with a radius per corner, as SVG path data. Each radius is clamped to
/// half the smaller side, and a radius of `0` gives a square corner (no arc is written).
///
/// # Arguments
///
/// - `rect` - The rectangle.
/// - `radii` - `[top_left, top_right, bottom_right, bottom_left]`, as in CSS `border-radius`;
///   pass `[r; 4]` for one radius. Negative radii count as `0`.
///
/// # Returns
///
/// The path data, closed.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{rounded_rect_path, Rect};
///
/// let square = Rect::new(0.0, 0.0, 10.0, 10.0);
/// assert_eq!(rounded_rect_path(square, [0.0; 4]), "M0 0 L10 0 L10 10 L0 10 L0 0 Z");
/// assert_eq!(
///     rounded_rect_path(square, [5.0, 0.0, 0.0, 0.0]),
///     "M5 0 L10 0 L10 10 L0 10 L0 5 A5 5 0 0 1 5 0 Z"
/// );
/// ```
#[must_use]
pub fn rounded_rect_path(rect: Rect, radii: [f64; 4]) -> String {
    let limit = rect.width.min(rect.height) / 2.0;
    let [tl, tr, br, bl] = radii.map(|r| r.max(0.0).min(limit));
    let (x, y) = (rect.x, rect.y);
    let (right, bottom) = (rect.right(), rect.bottom());
    let mut commands = vec![PathCommand::new('M', vec![x + tl, y])];
    let arc = |commands: &mut Vec<PathCommand>, r: f64, end: (f64, f64)| {
        if r > 0.0 {
            commands.push(PathCommand::new(
                'A',
                vec![r, r, 0.0, 0.0, 1.0, end.0, end.1],
            ));
        }
    };
    let line = |commands: &mut Vec<PathCommand>, to: (f64, f64)| {
        commands.push(PathCommand::new('L', vec![to.0, to.1]));
    };
    line(&mut commands, (right - tr, y));
    arc(&mut commands, tr, (right, y + tr));
    line(&mut commands, (right, bottom - br));
    arc(&mut commands, br, (right - br, bottom));
    line(&mut commands, (x + bl, bottom));
    arc(&mut commands, bl, (x, bottom - bl));
    line(&mut commands, (x, y + tl));
    arc(&mut commands, tl, (x + tl, y));
    commands.push(PathCommand::new('Z', Vec::new()));
    format_path(&commands, 6)
}

#[cfg(test)]
#[path = "rounded_rect_path.test.rs"]
mod tests;
