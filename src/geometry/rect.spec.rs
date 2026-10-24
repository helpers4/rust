// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn rect() -> impl Strategy<Value = Rect> {
    (-1000i32..1000, -1000i32..1000, 0i32..1000, 0i32..1000)
        .prop_map(|(x, y, w, h)| Rect::new(f64::from(x), f64::from(y), f64::from(w), f64::from(h)))
}

proptest! {
    #[test]
    fn the_union_contains_both_rectangles_and_does_not_depend_on_the_order(a in rect(), b in rect()) {
        let u = a.union(&b);
        prop_assert!(u.contains_rect(&a) && u.contains_rect(&b));
        prop_assert_eq!(u, b.union(&a));
    }

    #[test]
    fn the_intersection_is_symmetric_agrees_with_intersects_and_lies_inside_both(a in rect(), b in rect()) {
        prop_assert_eq!(a.intersection(&b), b.intersection(&a));
        prop_assert_eq!(a.intersects(&b), a.intersection(&b).is_some());
        if let Some(i) = a.intersection(&b) {
            prop_assert!(a.contains_rect(&i) && b.contains_rect(&i));
        }
    }

    #[test]
    fn a_non_empty_rectangle_contains_its_top_left_corner_never_its_bottom_right_one(r in rect().prop_filter("non-empty", |r| r.width > 0.0 && r.height > 0.0)) {
        prop_assert!(r.contains_point(Point::new(r.x, r.y)));
        prop_assert!(!r.contains_point(Point::new(r.right(), r.bottom())));
    }

    #[test]
    fn bounding_contains_every_rectangle(rects in prop::collection::vec(rect(), 1..8)) {
        let b = Rect::bounding(&rects);
        prop_assert!(b.is_some_and(|b| rects.iter().all(|r| b.contains_rect(r))));
    }
}
