// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn moving_in_two_steps_equals_moving_at_once(
        dx in -1000i32..1000,
        dy in -1000i32..1000,
        points in prop::collection::vec((-1000i32..1000, -1000i32..1000), 1..8)
    ) {
        let steps: Vec<String> = points[1..].iter().map(|(x, y)| format!(" l{x} {y}")).collect();
        let rest = steps.join("");
        let d = format!("M{} {}{rest}z", points[0].0, points[0].1);
        let (dx, dy) = (f64::from(dx), f64::from(dy));
        let once = transform_path(&d, Matrix::translate(dx, dy), 6).unwrap();
        let first = transform_path(&d, Matrix::translate(dx, 0.0), 6).unwrap();
        let twice = transform_path(&first, Matrix::translate(0.0, dy), 6).unwrap();
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn never_panics_on_any_text(d in "\\PC{0,40}") {
        let _ = transform_path(&d, Matrix::IDENTITY, 6);
    }
}
