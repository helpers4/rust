// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn points() -> impl Strategy<Value = Vec<Point>> {
    prop::collection::vec((-1000i32..1000, -1000i32..1000), 0..40).prop_map(|v| {
        v.into_iter()
            .map(|(x, y)| Point::new(f64::from(x), f64::from(y)))
            .collect()
    })
}

proptest! {
    #[test]
    fn keeps_a_subsequence_that_includes_the_ends(points in points(), tolerance in 0.0f64..50.0) {
        let kept = simplify_polyline(&points, tolerance).unwrap_or_default();
        prop_assert_eq!(kept.first(), points.first());
        prop_assert_eq!(kept.last(), points.last());
        let mut position = 0;
        for point in &kept {
            while points[position] != *point {
                position += 1;
            }
            position += 1;
        }
    }

    #[test]
    fn every_removed_point_is_within_tolerance_of_the_segment_that_replaced_it(points in points(), tolerance in 0.0f64..50.0) {
        let kept = simplify_polyline(&points, tolerance).unwrap_or_default();
        let mut next = 0;
        for point in &points {
            if kept.get(next) == Some(point) {
                next += 1;
            } else {
                let distance = distance_to_segment(*point, kept[next - 1], kept[next]);
                prop_assert!(distance <= tolerance + 1e-9);
            }
        }
    }
}
