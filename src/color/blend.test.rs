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

#[test]
fn lighten_and_hard_light_differ_from_normal() {
    // Backdrop 200, source 10: lighten keeps 200 (normal would give 10).
    assert_eq!(blend(gray(200), gray(10), BlendMode::Lighten).r(), 200);
    // Backdrop 200, source 51: hard light multiplies (normal would give 51).
    assert_eq!(blend(gray(200), gray(51), BlendMode::HardLight).r(), 80);
}

/// Values worked out apart from this code, from the W3C Compositing and Blending formulas, with
/// colors that clip at black, at white, at both or at neither.
#[test]
#[allow(clippy::too_many_lines)] // a table of cases
fn non_separable_modes_match_the_specification() {
    let cases = [
        ((255, 0, 0), (0, 0, 255), BlendMode::Hue, (54, 54, 255)),
        ((255, 0, 0), (0, 0, 255), BlendMode::Saturation, (255, 0, 0)),
        ((255, 0, 0), (0, 0, 255), BlendMode::Color, (54, 54, 255)),
        ((255, 0, 0), (0, 0, 255), BlendMode::Luminosity, (94, 0, 0)),
        (
            (240, 140, 40),
            (30, 150, 160),
            BlendMode::Hue,
            (28, 213, 228),
        ),
        (
            (240, 140, 40),
            (30, 150, 160),
            BlendMode::Saturation,
            (212, 147, 82),
        ),
        (
            (240, 140, 40),
            (30, 150, 160),
            BlendMode::Color,
            (74, 194, 204),
        ),
        (
            (240, 140, 40),
            (30, 150, 160),
            BlendMode::Luminosity,
            (193, 97, 0),
        ),
        ((20, 200, 40), (250, 30, 60), BlendMode::Hue, (252, 72, 96)),
        (
            (20, 200, 40),
            (250, 30, 60),
            BlendMode::Saturation,
            (0, 213, 24),
        ),
        (
            (20, 200, 40),
            (250, 30, 60),
            BlendMode::Color,
            (255, 70, 95),
        ),
        (
            (20, 200, 40),
            (250, 30, 60),
            BlendMode::Luminosity,
            (0, 165, 18),
        ),
        (
            (200, 200, 10),
            (10, 30, 240),
            BlendMode::Hue,
            (165, 172, 255),
        ),
        (
            (200, 200, 10),
            (10, 30, 240),
            BlendMode::Saturation,
            (201, 201, 0),
        ),
        (
            (200, 200, 10),
            (10, 30, 240),
            BlendMode::Color,
            (165, 172, 255),
        ),
        (
            (200, 200, 10),
            (10, 30, 240),
            BlendMode::Luminosity,
            (53, 53, 0),
        ),
        (
            (255, 255, 255),
            (255, 0, 0),
            BlendMode::Hue,
            (255, 255, 255),
        ),
        (
            (255, 255, 255),
            (255, 0, 0),
            BlendMode::Saturation,
            (255, 255, 255),
        ),
        (
            (255, 255, 255),
            (255, 0, 0),
            BlendMode::Color,
            (255, 255, 255),
        ),
        (
            (255, 255, 255),
            (255, 0, 0),
            BlendMode::Luminosity,
            (77, 77, 77),
        ),
        ((10, 10, 10), (0, 255, 0), BlendMode::Hue, (10, 10, 10)),
        (
            (10, 10, 10),
            (0, 255, 0),
            BlendMode::Saturation,
            (10, 10, 10),
        ),
        ((10, 10, 10), (0, 255, 0), BlendMode::Color, (0, 17, 0)),
        (
            (10, 10, 10),
            (0, 255, 0),
            BlendMode::Luminosity,
            (150, 150, 150),
        ),
        ((250, 250, 20), (5, 5, 200), BlendMode::Hue, (221, 221, 255)),
        (
            (250, 250, 20),
            (5, 5, 200),
            BlendMode::Saturation,
            (246, 246, 51),
        ),
        (
            (250, 250, 20),
            (5, 5, 200),
            BlendMode::Color,
            (221, 221, 255),
        ),
        (
            (250, 250, 20),
            (5, 5, 200),
            BlendMode::Luminosity,
            (30, 30, 0),
        ),
    ];
    for ((br, bg, bb), (sr, sg, sb), mode, (r, g, b)) in cases {
        let out = blend(Rgba::new(br, bg, bb, 255), Rgba::new(sr, sg, sb, 255), mode);
        assert_eq!(
            (out.r(), out.g(), out.b()),
            (r, g, b),
            "{mode:?} {:?} over {:?}",
            (sr, sg, sb),
            (br, bg, bb)
        );
    }
}

#[test]
fn black_and_white_survive_the_non_separable_modes() {
    let white = gray(255);
    let black = gray(0);
    for mode in [
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ] {
        assert_eq!(blend(white, white, mode), white);
        assert_eq!(blend(black, black, mode), black);
    }
}

#[test]
fn set_sat_scales_the_channels_above_the_smallest_one() {
    assert_eq!(set_sat([0.5, 0.75, 1.0], 0.5), [0.0, 0.25, 0.5]);
    assert_eq!(set_sat([0.5, 0.5, 0.5], 0.5), [0.0; 3]);
}

#[test]
fn clip_color_pulls_both_ends_back_in_range() {
    let low = clip_color([-0.2, 0.2, 0.6]);
    let high = clip_color([0.4, 0.8, 1.4]);
    for c in low.into_iter().chain(high) {
        assert!((-1e-12..=1.0 + 1e-12).contains(&c), "{c}");
    }
    assert_eq!(clip_color([0.0, 0.5, 1.0]), [0.0, 0.5, 1.0]);
}
