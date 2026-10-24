// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn new_sets_the_fields() {
    assert_eq!(
        Rect::new(1.0, 2.0, 3.0, 4.0),
        Rect {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0
        }
    );
}

#[test]
fn right_and_bottom_add_the_size() {
    let r = Rect::new(-1.0, -2.0, 3.0, 5.0);
    assert_eq!((r.right(), r.bottom()), (2.0, 3.0));
}

#[test]
fn default_is_empty_at_the_origin() {
    assert_eq!(Rect::default(), Rect::new(0.0, 0.0, 0.0, 0.0));
}

const SQUARE: Rect = Rect::new(0.0, 0.0, 10.0, 10.0);

#[test]
fn union_bounds_both_rectangles() {
    assert_eq!(
        SQUARE.union(&Rect::new(20.0, 5.0, 10.0, 10.0)),
        Rect::new(0.0, 0.0, 30.0, 15.0)
    );
    assert_eq!(
        SQUARE.union(&Rect::new(-5.0, -5.0, 1.0, 1.0)),
        Rect::new(-5.0, -5.0, 15.0, 15.0)
    );
    assert_eq!(SQUARE.union(&SQUARE), SQUARE);
}

#[test]
fn bounding_folds_a_slice_and_is_none_for_no_rectangle() {
    assert_eq!(Rect::bounding(&[]), None);
    assert_eq!(Rect::bounding(&[SQUARE]), Some(SQUARE));
    let group = [
        SQUARE,
        Rect::new(20.0, 5.0, 10.0, 10.0),
        Rect::new(-5.0, -5.0, 1.0, 1.0),
    ];
    assert_eq!(
        Rect::bounding(&group),
        Some(Rect::new(-5.0, -5.0, 35.0, 20.0))
    );
}

#[test]
fn intersection_is_the_overlapping_area() {
    assert_eq!(
        SQUARE.intersection(&Rect::new(5.0, 5.0, 10.0, 10.0)),
        Some(Rect::new(5.0, 5.0, 5.0, 5.0))
    );
    assert_eq!(
        SQUARE.intersection(&Rect::new(2.0, 3.0, 1.0, 1.0)),
        Some(Rect::new(2.0, 3.0, 1.0, 1.0))
    );
}

#[test]
fn touching_separate_or_empty_rectangles_do_not_intersect() {
    assert_eq!(SQUARE.intersection(&Rect::new(10.0, 0.0, 5.0, 5.0)), None);
    assert_eq!(SQUARE.intersection(&Rect::new(0.0, 10.0, 5.0, 5.0)), None);
    assert_eq!(SQUARE.intersection(&Rect::new(20.0, 20.0, 5.0, 5.0)), None);
    assert!(!SQUARE.intersects(&Rect::new(2.0, 2.0, 0.0, 5.0)));
    assert!(SQUARE.intersects(&Rect::new(5.0, 5.0, 10.0, 10.0)));
}

#[test]
fn contains_point_is_half_open() {
    assert!(SQUARE.contains_point(Point::new(5.0, 5.0)));
    assert!(SQUARE.contains_point(Point::new(0.0, 0.0)));
    assert!(!SQUARE.contains_point(Point::new(10.0, 5.0)));
    assert!(!SQUARE.contains_point(Point::new(5.0, 10.0)));
    assert!(!SQUARE.contains_point(Point::new(-1.0, 5.0)));
    assert!(!SQUARE.contains_point(Point::new(5.0, -1.0)));
}

#[test]
fn contains_rect_allows_touching_edges_but_not_sticking_out() {
    assert!(SQUARE.contains_rect(&SQUARE));
    assert!(SQUARE.contains_rect(&Rect::new(5.0, 5.0, 5.0, 5.0)));
    assert!(SQUARE.contains_rect(&Rect::new(3.0, 3.0, 0.0, 0.0)));
    assert!(!SQUARE.contains_rect(&Rect::new(-1.0, 0.0, 5.0, 5.0)));
    assert!(!SQUARE.contains_rect(&Rect::new(0.0, -1.0, 5.0, 5.0)));
    assert!(!SQUARE.contains_rect(&Rect::new(6.0, 0.0, 5.0, 5.0)));
    assert!(!SQUARE.contains_rect(&Rect::new(0.0, 6.0, 5.0, 5.0)));
}
