// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn gray(v: u8) -> Rgba {
    Rgba::new(v, v, v, 255)
}

#[test]
fn normal_composites_a_translucent_source_over_the_backdrop() {
    assert_eq!(
        blend(
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 128),
            BlendMode::Normal
        ),
        Rgba::new(127, 0, 128, 255)
    );
}

#[test]
fn an_opaque_source_replaces_the_backdrop_with_normal() {
    assert_eq!(
        blend(
            Rgba::new(1, 2, 3, 255),
            Rgba::new(9, 8, 7, 255),
            BlendMode::Normal
        ),
        Rgba::new(9, 8, 7, 255)
    );
}

#[test]
fn a_transparent_source_leaves_the_backdrop_alone() {
    assert_eq!(
        blend(
            Rgba::new(1, 2, 3, 100),
            Rgba::new(9, 8, 7, 0),
            BlendMode::Multiply
        ),
        Rgba::new(1, 2, 3, 100)
    );
}

#[test]
fn a_transparent_backdrop_gives_the_source() {
    assert_eq!(
        blend(
            Rgba::new(1, 2, 3, 0),
            Rgba::new(9, 8, 7, 128),
            BlendMode::Multiply
        ),
        Rgba::new(9, 8, 7, 128)
    );
}

#[test]
fn two_transparent_colors_give_transparent_black() {
    assert_eq!(
        blend(
            Rgba::new(1, 2, 3, 0),
            Rgba::new(9, 8, 7, 0),
            BlendMode::Normal
        ),
        Rgba::new(0, 0, 0, 0)
    );
}

#[test]
fn separable_modes_match_the_specification() {
    let cases = [
        (BlendMode::Multiply, 200, 100, 78),
        (BlendMode::Screen, 128, 128, 192),
        (BlendMode::Overlay, 128, 51, 52),
        (BlendMode::Overlay, 128, 204, 204),
        (BlendMode::Darken, 10, 200, 10),
        (BlendMode::Lighten, 10, 200, 200),
        (BlendMode::ColorDodge, 0, 128, 0),
        (BlendMode::ColorDodge, 128, 255, 255),
        (BlendMode::ColorDodge, 128, 128, 255),
        (BlendMode::ColorDodge, 51, 51, 64),
        (BlendMode::ColorBurn, 255, 128, 255),
        (BlendMode::ColorBurn, 128, 0, 0),
        (BlendMode::ColorBurn, 128, 128, 2),
        (BlendMode::ColorBurn, 204, 153, 170),
        (BlendMode::HardLight, 128, 51, 51),
        (BlendMode::HardLight, 128, 204, 204),
        (BlendMode::SoftLight, 51, 51, 27),
        (BlendMode::SoftLight, 51, 204, 89),
        (BlendMode::SoftLight, 128, 204, 160),
        (BlendMode::Difference, 200, 50, 150),
        (BlendMode::Exclusion, 200, 100, 143),
    ];
    for (mode, backdrop, source, expected) in cases {
        assert_eq!(
            blend(gray(backdrop), gray(source), mode).r(),
            expected,
            "{mode:?} {backdrop} {source}"
        );
    }
}

#[test]
fn blends_with_a_partly_transparent_backdrop() {
    assert_eq!(
        blend(
            Rgba::new(100, 100, 100, 128),
            Rgba::new(200, 200, 200, 128),
            BlendMode::Multiply
        ),
        Rgba::new(126, 126, 126, 192)
    );
}

fn luminosity(color: Rgba) -> f64 {
    let [r, g, b, _] = color.to_unit();
    0.3 * r + 0.59 * g + 0.11 * b
}

fn red() -> Rgba {
    Rgba::new(255, 0, 0, 255)
}
fn blue() -> Rgba {
    Rgba::new(0, 0, 255, 255)
}
fn orange() -> Rgba {
    Rgba::new(240, 140, 40, 255)
}
fn teal() -> Rgba {
    Rgba::new(30, 150, 160, 255)
}

#[test]
fn luminosity_keeps_the_backdrop_colors_with_the_source_luminosity() {
    let out = blend(red(), blue(), BlendMode::Luminosity);
    assert!((luminosity(out) - luminosity(blue())).abs() < 0.01);
    assert!(out.r() > out.g());
    let out = blend(orange(), teal(), BlendMode::Luminosity);
    assert!((luminosity(out) - luminosity(teal())).abs() < 0.01);
}

#[test]
fn color_keeps_the_backdrop_luminosity() {
    for (backdrop, source) in [(orange(), teal()), (red(), blue())] {
        let out = blend(backdrop, source, BlendMode::Color);
        assert!((luminosity(out) - luminosity(backdrop)).abs() < 0.01);
    }
}

#[test]
fn color_clips_what_goes_past_white() {
    assert_eq!(
        blend(gray(255), red(), BlendMode::Color),
        Rgba::new(255, 255, 255, 255)
    );
}

#[test]
fn hue_takes_the_source_hue_with_the_backdrop_saturation_and_luminosity() {
    let out = blend(orange(), teal(), BlendMode::Hue);
    assert!((luminosity(out) - luminosity(orange())).abs() < 0.01);
    let flat = blend(gray(90), red(), BlendMode::Hue);
    assert_eq!((flat.r(), flat.g()), (flat.g(), flat.b()));
}

#[test]
fn hue_with_a_gray_source_and_saturation_with_a_gray_backdrop_have_no_color() {
    let flat = blend(orange(), gray(100), BlendMode::Hue);
    assert_eq!((flat.r(), flat.g()), (flat.g(), flat.b()));
    let flat = blend(gray(100), teal(), BlendMode::Saturation);
    assert_eq!(flat.r(), flat.b());
}

#[test]
fn saturation_takes_the_source_saturation() {
    let out = blend(orange(), teal(), BlendMode::Saturation);
    assert!((luminosity(out) - luminosity(orange())).abs() < 0.01);
    let flat = blend(orange(), gray(70), BlendMode::Saturation);
    assert_eq!(flat.r(), flat.g());
}

#[test]
fn a_mode_that_mixes_channels_is_a_no_op_for_identical_colors() {
    for mode in [
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ] {
        let out = blend(orange(), orange(), mode);
        assert!(
            out.r().abs_diff(240) <= 1 && out.g().abs_diff(140) <= 1 && out.b().abs_diff(40) <= 1
        );
    }
}

#[test]
fn the_default_mode_is_normal() {
    assert_eq!(BlendMode::default(), BlendMode::Normal);
}
