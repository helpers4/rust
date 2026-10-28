// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_weight_is_always_a_css_weight(style in "\\PC{0,24}") {
        let weight = weight_of_style(&style);
        prop_assert!((100.0..=950.0).contains(&weight));
    }

    #[test]
    fn case_and_separators_never_change_the_weight(style in "[A-Za-z ]{0,16}") {
        prop_assert_eq!(
            weight_of_style(&style).to_bits(),
            weight_of_style(&style.to_uppercase().replace(' ', "-")).to_bits()
        );
    }
}
