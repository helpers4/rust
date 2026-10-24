// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Point;

/// Removes the points of a polyline that change its shape by less than `tolerance`, with the
/// Ramer–Douglas–Peucker algorithm: each removed point is at most `tolerance` away from the
/// simplified line. It shrinks map outlines or traced paths by orders of magnitude while keeping
/// their look.
///
/// The first and last points are always kept. With a `tolerance` of `0.0`, only points exactly on
/// a straight line are removed. It is iterative, so very long polylines cannot overflow the stack.
///
/// # Arguments
///
/// - `points` - The polyline.
/// - `tolerance` - The maximum distance between a removed point and the result, in the units of
///   the points.
///
/// # Returns
///
/// The kept points in their original order, or `None` when `tolerance` is negative or `NaN`.
///
/// # Examples
///
/// ```
/// use helpers4::geometry::{simplify_polyline, Point};
///
/// let line = [Point::new(0.0, 0.0), Point::new(1.0, 0.1), Point::new(2.0, 0.0)];
/// assert_eq!(
///     simplify_polyline(&line, 0.5),
///     Some(vec![Point::new(0.0, 0.0), Point::new(2.0, 0.0)])
/// );
/// assert_eq!(simplify_polyline(&line, -1.0), None);
/// ```
#[must_use]
pub fn simplify_polyline(points: &[Point], tolerance: f64) -> Option<Vec<Point>> {
    if tolerance.is_nan() || tolerance < 0.0 {
        return None;
    }
    if points.len() <= 2 {
        return Some(points.to_vec());
    }

    let last_index = points.len() - 1;
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[last_index] = true;
    let mut ranges = vec![(0, last_index)];
    while let Some((first, last)) = ranges.pop() {
        let mut farthest = None;
        let mut farthest_distance = tolerance;
        for index in first + 1..last {
            let distance = distance_to_segment(points[index], points[first], points[last]);
            // Strictly greater: a point exactly at the tolerance (or on the line when it is 0) goes.
            if distance > farthest_distance {
                farthest = Some(index);
                farthest_distance = distance;
            }
        }
        if let Some(index) = farthest {
            keep[index] = true;
            ranges.push((first, index));
            ranges.push((index, last));
        }
    }
    Some(
        points
            .iter()
            .zip(&keep)
            .filter(|(_, kept)| **kept)
            .map(|(point, _)| *point)
            .collect(),
    )
}

fn distance_to_segment(p: Point, a: Point, b: Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let length_squared = dx * dx + dy * dy;
    // The closest point of the segment: the projection, clamped to its ends (`a` when a and b coincide).
    let t = if length_squared == 0.0 {
        0.0
    } else {
        (((p.x - a.x) * dx + (p.y - a.y) * dy) / length_squared).clamp(0.0, 1.0)
    };
    (p.x - (a.x + t * dx)).hypot(p.y - (a.y + t * dy))
}

#[cfg(test)]
#[path = "simplify_polyline.test.rs"]
mod tests;

#[cfg(test)]
#[path = "simplify_polyline.spec.rs"]
mod spec;
