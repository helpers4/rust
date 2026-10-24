// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn size() -> impl Strategy<Value = Size> {
    (1u32..2000, 1u32..2000).prop_map(|(w, h)| Size::new(f64::from(w), f64::from(h)))
}

fn mode() -> impl Strategy<Value = FitMode> {
    prop_oneof![
        Just(FitMode::Contain),
        Just(FitMode::Cover),
        Just(FitMode::Fill),
        Just(FitMode::Original),
        Just(FitMode::ScaleDown),
    ]
}

proptest! {
    #[test]
    fn contain_and_scale_down_stay_inside_cover_and_fill_cover(content in size(), container in size(), mode in mode()) {
        let m = fit(content, container, mode, FitAlign::CENTER);
        let left = m.e;
        let top = m.f;
        let right = m.e + content.width * m.a;
        let bottom = m.f + content.height * m.d;
        let eps = 1e-6;
        if matches!(mode, FitMode::Contain | FitMode::ScaleDown) {
            prop_assert!(left >= -eps && top >= -eps);
            prop_assert!(right <= container.width + eps && bottom <= container.height + eps);
        }
        if matches!(mode, FitMode::Cover | FitMode::Fill) {
            prop_assert!(left <= eps && top <= eps);
            prop_assert!(right >= container.width - eps && bottom >= container.height - eps);
        }
    }

    #[test]
    fn centered_content_has_equal_margins(content in size(), container in size(), mode in mode()) {
        let m = fit(content, container, mode, FitAlign::CENTER);
        let right_margin = container.width - (m.e + content.width * m.a);
        let bottom_margin = container.height - (m.f + content.height * m.d);
        prop_assert!((m.e - right_margin).abs() < 1e-6);
        prop_assert!((m.f - bottom_margin).abs() < 1e-6);
    }
}
