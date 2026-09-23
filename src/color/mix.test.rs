// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn red() -> Rgb {
    Rgb::new(255, 0, 0)
}
fn blue() -> Rgb {
    Rgb::new(0, 0, 255)
}

#[test]
fn zero_gives_the_first_color_and_one_the_second() {
    assert_eq!(mix(red(), blue(), 0.0), red());
    assert_eq!(mix(red(), blue(), 1.0), blue());
}

#[test]
fn the_midpoint_rounds_half_up() {
    assert_eq!(mix(red(), blue(), 0.5), Rgb::new(128, 0, 128));
    assert_eq!(
        mix(Rgb::new(0, 0, 0), Rgb::new(100, 200, 50), 0.5),
        Rgb::new(50, 100, 25)
    );
}

#[test]
fn t_is_clamped() {
    assert_eq!(mix(red(), blue(), -3.0), red());
    assert_eq!(mix(red(), blue(), 3.0), blue());
}

#[test]
fn a_nan_counts_as_zero() {
    assert_eq!(mix(red(), blue(), f64::NAN), red());
}
