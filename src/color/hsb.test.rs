// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_the_primaries() {
    assert_eq!(Hsb::new(0.0, 1.0, 1.0).to_rgb(), Rgb::new(255, 0, 0));
    assert_eq!(Hsb::new(60.0, 1.0, 1.0).to_rgb(), Rgb::new(255, 255, 0));
    assert_eq!(Hsb::new(120.0, 1.0, 1.0).to_rgb(), Rgb::new(0, 255, 0));
    assert_eq!(Hsb::new(180.0, 1.0, 1.0).to_rgb(), Rgb::new(0, 255, 255));
    assert_eq!(Hsb::new(240.0, 1.0, 1.0).to_rgb(), Rgb::new(0, 0, 255));
    assert_eq!(Hsb::new(300.0, 1.0, 1.0).to_rgb(), Rgb::new(255, 0, 255));
}

#[test]
fn brightness_one_is_the_pure_color_not_white() {
    assert_eq!(Hsb::new(0.0, 0.0, 1.0).to_rgb(), Rgb::new(255, 255, 255));
    assert_eq!(Hsb::new(0.0, 1.0, 0.5).to_rgb(), Rgb::new(128, 0, 0));
}

#[test]
fn normalizes_its_input() {
    assert_eq!(Hsb::new(-90.0, 2.0, -1.0), Hsb::new(270.0, 1.0, 0.0));
    assert_eq!(
        Hsb::new(f64::NAN, f64::NAN, f64::NAN),
        Hsb::new(0.0, 0.0, 0.0)
    );
    let color = Hsb::new(10.0, 0.5, 0.25);
    assert_eq!((color.h(), color.s(), color.b()), (10.0, 0.5, 0.25));
}

#[test]
fn reads_the_hsb_of_an_rgb_color() {
    assert_eq!(Rgb::new(255, 0, 0).to_hsb(), Hsb::new(0.0, 1.0, 1.0));
    assert_eq!(
        Rgb::new(0, 128, 128).to_hsb(),
        Hsb::new(180.0, 1.0, 128.0 / 255.0)
    );
    assert_eq!(Rgb::new(0, 255, 0).to_hsb().h(), 120.0);
    assert_eq!(Rgb::new(0, 0, 255).to_hsb().h(), 240.0);
    assert_eq!(Rgb::new(255, 0, 128).to_hsb().h().round(), 330.0);
}

#[test]
fn a_gray_has_no_hue_or_saturation() {
    let gray = Rgb::new(51, 51, 51).to_hsb();
    assert_eq!((gray.h(), gray.s()), (0.0, 0.0));
    assert!((gray.b() - 0.2).abs() < 1e-12);
    assert_eq!(Rgb::new(0, 0, 0).to_hsb(), Hsb::new(0.0, 0.0, 0.0));
}

#[test]
fn round_trips_through_rgb_for_every_gray_and_a_spread_of_colors() {
    for v in 0..=255u8 {
        assert_eq!(Rgb::new(v, v, v).to_hsb().to_rgb(), Rgb::new(v, v, v));
    }
    for (r, g, b) in [
        (255, 0, 0),
        (12, 200, 99),
        (0, 128, 128),
        (250, 250, 3),
        (7, 7, 200),
    ] {
        assert_eq!(Rgb::new(r, g, b).to_hsb().to_rgb(), Rgb::new(r, g, b));
    }
}
