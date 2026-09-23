// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: normalization promises exact values at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn keeps_values_that_are_already_in_range() {
    let hsl = Hsl::new(200.0, 0.5, 0.25);
    assert_eq!((hsl.h(), hsl.s(), hsl.l()), (200.0, 0.5, 0.25));
}

#[test]
fn wraps_the_hue_around_360() {
    assert_eq!(Hsl::new(360.0, 0.5, 0.5).h(), 0.0);
    assert_eq!(Hsl::new(450.0, 0.5, 0.5).h(), 90.0);
    assert_eq!(Hsl::new(-90.0, 0.5, 0.5).h(), 270.0);
}

#[test]
fn clamps_saturation_and_lightness() {
    let hsl = Hsl::new(0.0, 2.0, -1.0);
    assert_eq!((hsl.s(), hsl.l()), (1.0, 0.0));
    let hsl = Hsl::new(0.0, -2.0, 3.0);
    assert_eq!((hsl.s(), hsl.l()), (0.0, 1.0));
}

#[test]
fn a_nan_counts_as_zero() {
    let hsl = Hsl::new(f64::NAN, f64::NAN, f64::NAN);
    assert_eq!((hsl.h(), hsl.s(), hsl.l()), (0.0, 0.0, 0.0));
}

#[test]
fn converts_every_hue_sector_to_rgb() {
    let rgb = |h| Hsl::new(h, 1.0, 0.5).to_rgb();
    assert_eq!(rgb(0.0), Rgb::new(255, 0, 0));
    assert_eq!(rgb(30.0), Rgb::new(255, 128, 0));
    assert_eq!(rgb(60.0), Rgb::new(255, 255, 0));
    assert_eq!(rgb(90.0), Rgb::new(128, 255, 0));
    assert_eq!(rgb(120.0), Rgb::new(0, 255, 0));
    assert_eq!(rgb(150.0), Rgb::new(0, 255, 128));
    assert_eq!(rgb(180.0), Rgb::new(0, 255, 255));
    assert_eq!(rgb(210.0), Rgb::new(0, 128, 255));
    assert_eq!(rgb(240.0), Rgb::new(0, 0, 255));
    assert_eq!(rgb(270.0), Rgb::new(128, 0, 255));
    assert_eq!(rgb(300.0), Rgb::new(255, 0, 255));
    assert_eq!(rgb(330.0), Rgb::new(255, 0, 128));
}

#[test]
fn zero_saturation_is_a_grey_and_the_ends_are_black_and_white() {
    assert_eq!(Hsl::new(123.0, 0.0, 0.5).to_rgb(), Rgb::new(128, 128, 128));
    assert_eq!(Hsl::new(123.0, 1.0, 0.0).to_rgb(), Rgb::new(0, 0, 0));
    assert_eq!(Hsl::new(123.0, 1.0, 1.0).to_rgb(), Rgb::new(255, 255, 255));
}
