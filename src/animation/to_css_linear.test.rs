// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn samples_evenly_in_time() {
    assert_eq!(
        to_css_linear(&|t| t * t, 5).as_deref(),
        Some("linear(0, 0.0625, 0.25, 0.5625, 1)")
    );
}

#[test]
fn rounds_to_four_decimals_and_writes_no_negative_zero() {
    assert_eq!(
        to_css_linear(&|t| if t == 0.0 { -0.00001 } else { 1.0 / 3.0 }, 2).as_deref(),
        Some("linear(0, 0.3333)")
    );
}

#[test]
fn needs_at_least_two_samples() {
    assert_eq!(to_css_linear(&|t| t, 0), None);
    assert_eq!(to_css_linear(&|t| t, 1), None);
    assert!(to_css_linear(&|t| t, 2).is_some());
}

#[test]
fn the_default_count_of_32_gives_32_values() {
    let css = to_css_linear(&|t| t, 32).unwrap_or_default();
    assert_eq!(css.matches(',').count(), 31);
}
