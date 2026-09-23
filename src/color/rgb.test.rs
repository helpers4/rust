// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these conversions promise exact values at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

fn rgb(text: &str) -> Rgb {
    Rgb::parse(text).unwrap()
}

#[test]
fn parses_long_and_short_hex() {
    assert_eq!(rgb("#ff8800"), Rgb::new(255, 136, 0));
    assert_eq!(rgb("#f80"), Rgb::new(255, 136, 0));
    assert_eq!(rgb("#000000"), Rgb::new(0, 0, 0));
    assert_eq!(rgb("#FFFFFF"), Rgb::new(255, 255, 255));
    assert_eq!(rgb("#aBcDeF"), Rgb::new(0xab, 0xcd, 0xef));
    assert_eq!(rgb("#123"), Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn parses_the_rgb_function() {
    assert_eq!(rgb("rgb(0, 128, 255)"), Rgb::new(0, 128, 255));
    assert_eq!(rgb("rgb(0 128 255)"), Rgb::new(0, 128, 255));
    assert_eq!(rgb("RGB( 1,2 , 3 )"), Rgb::new(1, 2, 3));
    assert_eq!(rgb("rgb(255,255,255)"), Rgb::new(255, 255, 255));
}

#[test]
fn parses_every_basic_color_name() {
    for (name, hex) in [
        ("black", "#000000"),
        ("silver", "#c0c0c0"),
        ("gray", "#808080"),
        ("white", "#ffffff"),
        ("maroon", "#800000"),
        ("red", "#ff0000"),
        ("purple", "#800080"),
        ("fuchsia", "#ff00ff"),
        ("green", "#008000"),
        ("lime", "#00ff00"),
        ("olive", "#808000"),
        ("yellow", "#ffff00"),
        ("navy", "#000080"),
        ("blue", "#0000ff"),
        ("teal", "#008080"),
        ("aqua", "#00ffff"),
        ("orange", "#ffa500"),
    ] {
        assert_eq!(rgb(name).to_hex(), hex, "{name}");
        assert_eq!(rgb(&name.to_uppercase()).to_hex(), hex);
    }
}

#[test]
fn ignores_surrounding_whitespace() {
    assert_eq!(rgb("  #fff\n"), Rgb::new(255, 255, 255));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(Rgb::parse(""), Err(ParseColorError::Empty));
    assert_eq!(Rgb::parse("   "), Err(ParseColorError::Empty));
}

#[test]
fn rejects_hex_of_the_wrong_length() {
    assert_eq!(
        Rgb::parse("#"),
        Err(ParseColorError::InvalidLength { length: 0 })
    );
    assert_eq!(
        Rgb::parse("#ff"),
        Err(ParseColorError::InvalidLength { length: 2 })
    );
    assert_eq!(
        Rgb::parse("#ffff"),
        Err(ParseColorError::InvalidLength { length: 4 })
    );
    assert_eq!(
        Rgb::parse("#ff880000"),
        Err(ParseColorError::InvalidLength { length: 8 })
    );
}

#[test]
fn rejects_a_non_hex_digit() {
    assert_eq!(
        Rgb::parse("#gg0000"),
        Err(ParseColorError::InvalidDigit { index: 1 })
    );
    assert_eq!(
        Rgb::parse("#12z"),
        Err(ParseColorError::InvalidDigit { index: 3 })
    );
}

#[test]
fn rejects_a_malformed_rgb_function() {
    for text in [
        "rgb()",
        "rgb(1, 2)",
        "rgb(1, 2, 3, 4)",
        "rgb(a, b, c)",
        "rgb(1.5, 2, 3)",
        "rgb(-1, 2, 3)",
        "rgb(1, 2, 3",
    ] {
        assert!(Rgb::parse(text).is_err(), "{text:?}");
    }
    assert_eq!(Rgb::parse("rgb()"), Err(ParseColorError::InvalidFunction));
    assert_eq!(
        Rgb::parse("rgb(1, 2, x)"),
        Err(ParseColorError::InvalidFunction)
    );
    assert_eq!(
        Rgb::parse("rgb(1, x, 3)"),
        Err(ParseColorError::InvalidFunction)
    );
    assert_eq!(
        Rgb::parse("rgb(x, 2, 3)"),
        Err(ParseColorError::InvalidFunction)
    );
}

#[test]
fn rejects_a_channel_above_255() {
    assert_eq!(
        Rgb::parse("rgb(256, 0, 0)"),
        Err(ParseColorError::OutOfRange)
    );
    assert_eq!(
        Rgb::parse("rgb(0, 300, 0)"),
        Err(ParseColorError::OutOfRange)
    );
    assert_eq!(
        Rgb::parse("rgb(0, 0, 65535)"),
        Err(ParseColorError::OutOfRange)
    );
    assert_eq!(
        Rgb::parse("rgb(0, 0, 65536)"),
        Err(ParseColorError::InvalidFunction)
    );
}

#[test]
fn rejects_unknown_names_and_bare_hex() {
    assert_eq!(Rgb::parse("notacolor"), Err(ParseColorError::UnknownName));
    assert_eq!(Rgb::parse("ff8800"), Err(ParseColorError::UnknownName));
    assert_eq!(
        Rgb::parse("hsl(0, 0%, 0%)"),
        Err(ParseColorError::UnknownName)
    );
}

#[test]
fn writes_lowercase_hex() {
    assert_eq!(Rgb::new(255, 136, 0).to_hex(), "#ff8800");
    assert_eq!(Rgb::new(1, 2, 3).to_string(), "#010203");
    assert_eq!("#F80".parse::<Rgb>(), Ok(Rgb::new(255, 136, 0)));
}

#[test]
fn converts_the_primary_colors_to_hsl() {
    let hsl = |r, g, b| Rgb::new(r, g, b).to_hsl();
    assert_eq!(
        (hsl(255, 0, 0).h(), hsl(255, 0, 0).s(), hsl(255, 0, 0).l()),
        (0.0, 1.0, 0.5)
    );
    assert_eq!((hsl(0, 255, 0).h(), hsl(0, 255, 0).s()), (120.0, 1.0));
    assert_eq!((hsl(0, 0, 255).h(), hsl(0, 0, 255).s()), (240.0, 1.0));
    assert_eq!(hsl(255, 255, 0).h(), 60.0);
    assert_eq!(hsl(0, 255, 255).h(), 180.0);
    assert_eq!(hsl(255, 0, 255).h(), 300.0);
}

#[test]
fn greys_have_no_hue_or_saturation() {
    let hsl = Rgb::new(128, 128, 128).to_hsl();
    assert_eq!((hsl.h(), hsl.s()), (0.0, 0.0));
    assert!((hsl.l() - 128.0 / 255.0).abs() < 1e-12);
    assert_eq!(Rgb::new(0, 0, 0).to_hsl().l(), 0.0);
    assert_eq!(Rgb::new(255, 255, 255).to_hsl().l(), 1.0);
}

#[test]
fn a_hue_just_below_red_wraps_around() {
    // Red with a little blue is at hue ~ 350, not a negative number.
    let hsl = Rgb::new(255, 0, 26).to_hsl();
    assert!(hsl.h() > 340.0 && hsl.h() < 360.0, "{}", hsl.h());
}

#[test]
fn luminance_runs_from_black_to_white() {
    assert_eq!(Rgb::new(0, 0, 0).luminance(), 0.0);
    assert!((Rgb::new(255, 255, 255).luminance() - 1.0).abs() < 1e-12);
    assert!((Rgb::new(255, 0, 0).luminance() - 0.2126).abs() < 1e-12);
    assert!((Rgb::new(0, 255, 0).luminance() - 0.7152).abs() < 1e-12);
    assert!((Rgb::new(0, 0, 255).luminance() - 0.0722).abs() < 1e-12);
}

#[test]
fn dark_channels_use_the_linear_part_of_the_curve() {
    // 10/255 is below the 0.04045 threshold: value / 12.92 / 255.
    let expected = 0.2126 * (10.0 / 255.0 / 12.92);
    assert!((Rgb::new(10, 0, 0).luminance() - expected).abs() < 1e-12);
}

#[test]
fn lightens_and_darkens_towards_white_and_black() {
    assert_eq!(
        Rgb::new(100, 100, 100).lighten(0.0),
        Rgb::new(100, 100, 100)
    );
    assert_eq!(
        Rgb::new(100, 100, 100).lighten(1.0),
        Rgb::new(255, 255, 255)
    );
    assert_eq!(Rgb::new(100, 100, 100).darken(1.0), Rgb::new(0, 0, 0));
    assert_eq!(Rgb::new(100, 200, 0).lighten(0.5), Rgb::new(178, 228, 128));
    assert_eq!(Rgb::new(100, 200, 0).darken(0.5), Rgb::new(50, 100, 0));
    assert_eq!(
        Rgb::new(100, 100, 100).lighten(2.0),
        Rgb::new(255, 255, 255)
    );
}
