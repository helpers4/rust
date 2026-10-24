// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    #[allow(clippy::float_cmp)] // the ends of a curve are pinned exactly, by design
    fn cubic_bezier_is_finite_for_any_input_and_pins_the_ends(x1 in 0.0f64..=1.0, y1 in -2.0f64..3.0, x2 in 0.0f64..=1.0, y2 in -2.0f64..3.0, t in any::<f64>()) {
        let easing = Easing::cubic_bezier(x1, y1, x2, y2).unwrap();
        prop_assert!(easing.at(t).is_finite());
        prop_assert_eq!(easing.at(0.0), 0.0);
        prop_assert_eq!(easing.at(1.0), 1.0);
    }

    #[test]
    fn cubic_bezier_never_decreases_when_y_stays_in_the_unit_range(x1 in 0.0f64..=1.0, y1 in 0.0f64..=1.0, x2 in 0.0f64..=1.0, y2 in 0.0f64..=1.0, a in 0.0f64..=1.0, b in 0.0f64..=1.0) {
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        let easing = Easing::cubic_bezier(x1, y1, x2, y2);
        prop_assert!(easing.is_some_and(|e| e.at(low) <= e.at(high) + 1e-8));
    }

    #[test]
    #[allow(clippy::float_cmp)] // the ends of a curve are pinned exactly, by design
    fn steps_never_decrease_stay_in_the_unit_range_and_end_at_one(count in 1u32..20, a in 0.0f64..=1.0, b in 0.0f64..=1.0) {
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        for position in [StepPosition::JumpStart, StepPosition::JumpEnd, StepPosition::JumpBoth] {
            let easing = Easing::steps(count, position).unwrap();
            prop_assert!(easing.at(low) <= easing.at(high));
            prop_assert!((0.0..=1.0).contains(&easing.at(low)));
            prop_assert_eq!(easing.at(1.0), 1.0);
        }
    }

    #[test]
    fn at_never_panics(t in any::<f64>()) {
        let _ = Easing::linear().at(t);
    }
}
