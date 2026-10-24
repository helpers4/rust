// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn p(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

#[test]
fn removes_a_point_closer_than_the_tolerance() {
    assert_eq!(
        simplify_polyline(&[p(0.0, 0.0), p(1.0, 0.1), p(2.0, 0.0)], 0.5),
        Some(vec![p(0.0, 0.0), p(2.0, 0.0)])
    );
}

#[test]
fn keeps_a_point_farther_than_the_tolerance() {
    let line = [p(0.0, 0.0), p(1.0, 1.0), p(2.0, 0.0)];
    assert_eq!(simplify_polyline(&line, 0.5), Some(line.to_vec()));
}

#[test]
fn removes_a_point_exactly_at_the_tolerance() {
    assert_eq!(
        simplify_polyline(&[p(0.0, 0.0), p(1.0, 0.5), p(2.0, 0.0)], 0.5),
        Some(vec![p(0.0, 0.0), p(2.0, 0.0)])
    );
}

#[test]
fn with_a_tolerance_of_zero_only_collinear_points_go() {
    assert_eq!(
        simplify_polyline(&[p(0.0, 0.0), p(1.0, 0.0), p(2.0, 0.0), p(3.0, 1.0)], 0.0),
        Some(vec![p(0.0, 0.0), p(2.0, 0.0), p(3.0, 1.0)])
    );
}

#[test]
fn keeps_the_corners_of_a_closed_outline() {
    let square = [
        p(0.0, 0.0),
        p(5.0, 0.0),
        p(10.0, 0.0),
        p(10.0, 10.0),
        p(0.0, 10.0),
        p(0.0, 0.0),
    ];
    assert_eq!(
        simplify_polyline(&square, 1.0),
        Some(vec![
            p(0.0, 0.0),
            p(10.0, 0.0),
            p(10.0, 10.0),
            p(0.0, 10.0),
            p(0.0, 0.0)
        ])
    );
}

#[test]
fn measures_the_distance_to_the_segment_ends_beyond_the_segment() {
    let line = [p(0.0, 0.0), p(-3.0, 0.0), p(1.0, 0.0)];
    assert_eq!(simplify_polyline(&line, 1.0), Some(line.to_vec()));
}

#[test]
fn a_closed_loop_with_coinciding_ends_keeps_its_far_points() {
    let loop_ = [p(0.0, 0.0), p(5.0, 5.0), p(0.0, 0.0)];
    assert_eq!(simplify_polyline(&loop_, 1.0), Some(loop_.to_vec()));
    assert_eq!(
        simplify_polyline(&[p(0.0, 0.0), p(0.2, 0.1), p(0.0, 0.0)], 1.0),
        Some(vec![p(0.0, 0.0), p(0.0, 0.0)])
    );
}

#[test]
fn short_polylines_are_returned_as_they_are() {
    assert_eq!(simplify_polyline(&[], 1.0), Some(vec![]));
    assert_eq!(
        simplify_polyline(&[p(1.0, 1.0)], 1.0),
        Some(vec![p(1.0, 1.0)])
    );
    assert_eq!(
        simplify_polyline(&[p(0.0, 0.0), p(1.0, 1.0)], 1.0),
        Some(vec![p(0.0, 0.0), p(1.0, 1.0)])
    );
}

#[test]
fn refuses_a_negative_or_nan_tolerance() {
    assert_eq!(simplify_polyline(&[], -1.0), None);
    assert_eq!(simplify_polyline(&[], f64::NAN), None);
}

#[test]
fn handles_long_polylines() {
    let zigzag: Vec<Point> = (0..5_000u32)
        .map(|i| p(f64::from(i), f64::from(i % 2)))
        .collect();
    assert_eq!(
        simplify_polyline(&zigzag, 2.0),
        Some(vec![p(0.0, 0.0), p(4_999.0, 1.0)])
    );
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn the_distance_to_a_segment_is_exact_away_from_the_origin() {
    // A 3-4-5 segment from (1, 1) to (4, 5): the point (4, 1) projects inside it.
    assert!(close(
        distance_to_segment(p(4.0, 1.0), p(1.0, 1.0), p(4.0, 5.0)),
        2.4
    ));
    // The same segment with the point on the other side, and past either end.
    assert!(close(
        distance_to_segment(p(1.0, 5.0), p(1.0, 1.0), p(4.0, 5.0)),
        2.4
    ));
    assert!(close(
        distance_to_segment(p(7.0, 9.0), p(1.0, 1.0), p(4.0, 5.0)),
        5.0
    ));
    assert!(close(
        distance_to_segment(p(-2.0, -3.0), p(1.0, 1.0), p(4.0, 5.0)),
        5.0
    ));
    // On the segment.
    assert!(close(
        distance_to_segment(p(2.5, 3.0), p(1.0, 1.0), p(4.0, 5.0)),
        0.0
    ));
}

#[test]
fn the_distance_to_a_segment_handles_negative_coordinates_and_a_falling_slope() {
    // From (-3, 2) to (1, -4), seen from the origin: |4·(2 − 0) − (−3 − 0)·(−6)| / √52.
    let expected = (4.0f64 * 2.0 - 18.0).abs() / 52.0f64.sqrt();
    assert!(close(
        distance_to_segment(p(0.0, 0.0), p(-3.0, 2.0), p(1.0, -4.0)),
        expected
    ));
}

#[test]
fn the_distance_to_a_segment_of_zero_length_is_the_distance_to_its_point() {
    assert!(close(
        distance_to_segment(p(5.0, 6.0), p(2.0, 2.0), p(2.0, 2.0)),
        5.0
    ));
    assert!(close(
        distance_to_segment(p(2.0, 2.0), p(2.0, 2.0), p(2.0, 2.0)),
        0.0
    ));
}
