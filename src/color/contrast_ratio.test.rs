// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: the ratio is symmetric bit for bit.
#![allow(clippy::float_cmp)]

use super::*;

fn black() -> Rgb {
    Rgb::new(0, 0, 0)
}
fn white() -> Rgb {
    Rgb::new(255, 255, 255)
}

#[test]
fn black_on_white_is_the_maximum() {
    assert!((contrast_ratio(black(), white()) - 21.0).abs() < 1e-9);
}

#[test]
fn the_same_color_is_the_minimum() {
    assert!((contrast_ratio(white(), white()) - 1.0).abs() < 1e-12);
    assert!((contrast_ratio(black(), black()) - 1.0).abs() < 1e-12);
}

#[test]
fn the_order_of_the_colors_does_not_matter() {
    let grey = Rgb::new(118, 118, 118);
    assert_eq!(contrast_ratio(grey, white()), contrast_ratio(white(), grey));
}

#[test]
fn matches_the_known_aa_threshold_grey() {
    // #767676 is the lightest grey that reaches 4.5:1 on white (about 4.54).
    let ratio = contrast_ratio(Rgb::new(0x76, 0x76, 0x76), white());
    assert!(ratio > 4.5 && ratio < 4.6, "{ratio}");
    assert!(contrast_ratio(Rgb::new(0x77, 0x77, 0x77), white()) < ratio);
}
