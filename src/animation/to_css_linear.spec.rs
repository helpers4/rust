// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::animation::Easing;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_one_value_per_sample_all_numbers(samples in 2u32..100) {
        let css = to_css_linear(&|t| (t * 3.0).sin(), samples).unwrap_or_default();
        let inner = css.strip_prefix("linear(").and_then(|rest| rest.strip_suffix(')')).unwrap_or_default();
        let values: Vec<f64> = inner.split(", ").filter_map(|v| v.parse().ok()).collect();
        prop_assert_eq!(values.len(), samples as usize);
    }

    #[test]
    fn reads_back_as_the_same_curve_at_the_sample_points(samples in 2u32..40) {
        let curve = |t: f64| t * t;
        let css = to_css_linear(&curve, samples).unwrap_or_default();
        let easing = Easing::parse(&css).unwrap_or_else(|_| Easing::linear());
        for index in 0..samples {
            let t = f64::from(index) / f64::from(samples - 1);
            prop_assert!((easing.at(t) - curve(t)).abs() < 1e-3);
        }
    }
}
