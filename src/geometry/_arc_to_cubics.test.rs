// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::f64::consts::PI;

const K_QUARTER: f64 = 0.552_284_749_830_793_6;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn assert_cubic(actual: &PathCommand, expected: [f64; 6]) {
    assert_eq!(actual.letter, 'C');
    for (got, want) in actual.values.iter().zip(expected) {
        assert!(close(*got, want), "{:?} vs {expected:?}", actual.values);
    }
}

/// The point of an ellipse at parameter `theta`, and its derivative.
fn on_ellipse(
    center: (f64, f64),
    radii: (f64, f64),
    rotation: f64,
    theta: f64,
) -> ((f64, f64), (f64, f64)) {
    let (sin, cos) = rotation.to_radians().sin_cos();
    let at = |x: f64, y: f64| (center.0 + cos * x - sin * y, center.1 + sin * x + cos * y);
    let point = at(radii.0 * theta.cos(), radii.1 * theta.sin());
    // The derivative has no center.
    let slope = at(-radii.0 * theta.sin(), radii.1 * theta.cos());
    (point, (slope.0 - center.0, slope.1 - center.1))
}

/// What the arc from parameter `start` through `sweep_angle` must be, computed from the
/// parametric ellipse (the standard four-thirds tangent rule per quarter turn at most).
fn expected(
    center: (f64, f64),
    radii: (f64, f64),
    rotation: f64,
    start: f64,
    sweep_angle: f64,
) -> Vec<[f64; 6]> {
    let count = (sweep_angle.abs() / (PI / 2.0) - 1e-9).ceil().max(1.0);
    let step = sweep_angle / count;
    let k = 4.0 / 3.0 * (step / 4.0).tan();
    let mut out = Vec::new();
    let mut a = start;
    for _ in 0..count as usize {
        let b = a + step;
        let (p0, d0) = on_ellipse(center, radii, rotation, a);
        let (p1, d1) = on_ellipse(center, radii, rotation, b);
        out.push([
            p0.0 + k * d0.0,
            p0.1 + k * d0.1,
            p1.0 - k * d1.0,
            p1.1 - k * d1.1,
            p1.0,
            p1.1,
        ]);
        a = b;
    }
    out
}

/// Checks the arc of an ellipse against the oracle: the end points are given by the parameters,
/// and the flags are what the SVG endpoint form derives from them.
fn check(center: (f64, f64), radii: (f64, f64), rotation: f64, start: f64, sweep_angle: f64) {
    let (from, _) = on_ellipse(center, radii, rotation, start);
    let (to, _) = on_ellipse(center, radii, rotation, start + sweep_angle);
    let large = sweep_angle.abs() > PI;
    let sweep = sweep_angle > 0.0;
    let actual = arc_to_cubics(from, radii, rotation, large, sweep, to);
    let wanted = expected(center, radii, rotation, start, sweep_angle);
    assert_eq!(
        actual.len(),
        wanted.len(),
        "{center:?} {radii:?} {rotation}"
    );
    for (got, want) in actual.iter().zip(wanted) {
        assert_cubic(got, want);
    }
}

#[test]
fn a_quarter_circle_uses_the_standard_control_points() {
    let arc = arc_to_cubics((5.0, 0.0), (5.0, 5.0), 0.0, false, true, (0.0, 5.0));
    assert_eq!(arc.len(), 1);
    assert_cubic(
        &arc[0],
        [5.0, 5.0 * K_QUARTER, 5.0 * K_QUARTER, 5.0, 0.0, 5.0],
    );
}

#[test]
fn a_rotated_ellipse_quarter_follows_its_axes() {
    // Center (0, 0), radii (10, 5), turned 90 degrees: the point at parameter t is
    // (-5 sin t, 10 cos t).
    let arc = arc_to_cubics((0.0, 10.0), (10.0, 5.0), 90.0, false, true, (-5.0, 0.0));
    assert_eq!(arc.len(), 1);
    assert_cubic(
        &arc[0],
        [-5.0 * K_QUARTER, 10.0, -5.0, 10.0 * K_QUARTER, -5.0, 0.0],
    );
}

#[test]
fn a_large_arc_is_cut_in_quarter_turns() {
    let arc = arc_to_cubics((5.0, 0.0), (5.0, 5.0), 0.0, true, true, (0.0, -5.0));
    assert_eq!(arc.len(), 3);
    let ends: Vec<(f64, f64)> = arc.iter().map(|c| (c.values[4], c.values[5])).collect();
    assert!(close(ends[0].0, 0.0) && close(ends[0].1, 5.0));
    assert!(close(ends[1].0, -5.0) && close(ends[1].1, 0.0));
    assert_eq!(ends[2], (0.0, -5.0));
}

#[test]
fn matches_the_parametric_ellipse_in_every_direction_and_size() {
    let centers = [(0.0, 0.0), (7.0, -3.0)];
    let radii = [(5.0, 5.0), (10.0, 4.0), (3.0, 8.0)];
    let rotations = [0.0, 30.0, -75.0, 90.0];
    let starts = [0.0, 1.0, -2.5, 3.0];
    let sweeps = [0.5, 1.4, 3.0, 4.2, 5.9, -0.7, -2.0, -3.3, -4.6, -5.8];
    for center in centers {
        for radii in radii {
            for rotation in rotations {
                for start in starts {
                    for sweep_angle in sweeps {
                        check(center, radii, rotation, start, sweep_angle);
                    }
                }
            }
        }
    }
}

#[test]
fn radii_that_are_too_small_are_scaled_up_to_reach_the_end_point() {
    // Radius 1 cannot span 10: it becomes 5, a half circle centered at (5, 0), here swept
    // through (5, -5).
    let arc = arc_to_cubics((0.0, 0.0), (1.0, 1.0), 0.0, false, true, (10.0, 0.0));
    assert_eq!(arc.len(), 2);
    assert!(close(arc[0].values[4], 5.0) && close(arc[0].values[5], -5.0));
    assert_eq!((arc[1].values[4], arc[1].values[5]), (10.0, 0.0));
    // Radii that exactly span the chord are left alone.
    let exact = arc_to_cubics((0.0, 0.0), (5.0, 5.0), 0.0, false, true, (10.0, 0.0));
    assert!(close(exact[0].values[5], -5.0));
    // Negative radii count as their absolute value.
    let negative = arc_to_cubics((5.0, 0.0), (-5.0, -5.0), 0.0, false, true, (0.0, 5.0));
    assert_cubic(
        &negative[0],
        [5.0, 5.0 * K_QUARTER, 5.0 * K_QUARTER, 5.0, 0.0, 5.0],
    );
}
