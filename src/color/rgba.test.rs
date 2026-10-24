// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn builds_and_reads_back_the_channels() {
    let c = Rgba::new(1, 2, 3, 4);
    assert_eq!((c.r(), c.g(), c.b(), c.a()), (1, 2, 3, 4));
    assert_eq!(Rgba::from_rgb(Rgb::new(1, 2, 3), 4), c);
    assert_eq!(Rgba::from(Rgb::new(1, 2, 3)), Rgba::new(1, 2, 3, 255));
    assert_eq!(c.to_rgb(), Rgb::new(1, 2, 3));
}

#[test]
fn opacity_goes_from_zero_to_one() {
    assert!((Rgba::new(0, 0, 0, 255).opacity() - 1.0).abs() < f64::EPSILON);
    assert!(Rgba::new(0, 0, 0, 0).opacity().abs() < f64::EPSILON);
    assert!((Rgba::new(0, 0, 0, 51).opacity() - 0.2).abs() < 1e-9);
}

#[test]
fn parses_hex_with_alpha() {
    assert_eq!(Rgba::parse("#f808"), Ok(Rgba::new(255, 136, 0, 136)));
    assert_eq!(Rgba::parse("#FF880080"), Ok(Rgba::new(255, 136, 0, 128)));
    assert_eq!(Rgba::parse("  #ff8800ff "), Ok(Rgba::new(255, 136, 0, 255)));
}

#[test]
fn reads_every_digit_of_hex_with_alpha_in_its_own_place() {
    // No zero digit, so no operation on a digit can hide behind a zero.
    assert_eq!(Rgba::parse("#1234"), Ok(Rgba::new(17, 34, 51, 68)));
    assert_eq!(Rgba::parse("#12345678"), Ok(Rgba::new(18, 52, 86, 120)));
    assert_eq!(Rgba::parse("#abcdef12"), Ok(Rgba::new(171, 205, 239, 18)));
}

#[test]
fn rejects_a_bad_digit_in_hex_with_alpha() {
    assert_eq!(
        Rgba::parse("#ff88zz80"),
        Err(ParseColorError::InvalidDigit { index: 5 })
    );
    assert_eq!(
        Rgba::parse("#f80g"),
        Err(ParseColorError::InvalidDigit { index: 4 })
    );
}

#[test]
fn reads_every_opaque_form_of_rgb_as_fully_opaque() {
    assert_eq!(Rgba::parse("#f80"), Ok(Rgba::new(255, 136, 0, 255)));
    assert_eq!(Rgba::parse("#ff8800"), Ok(Rgba::new(255, 136, 0, 255)));
    assert_eq!(Rgba::parse("rgb(1, 2, 3)"), Ok(Rgba::new(1, 2, 3, 255)));
    assert_eq!(Rgba::parse("rgba(1 2 3)"), Ok(Rgba::new(1, 2, 3, 255)));
    assert_eq!(Rgba::parse("Red"), Ok(Rgba::new(255, 0, 0, 255)));
}

#[test]
fn parses_functions_with_alpha_in_both_syntaxes() {
    assert_eq!(
        Rgba::parse("rgb(255 136 0 / 0.5)"),
        Ok(Rgba::new(255, 136, 0, 128))
    );
    assert_eq!(
        Rgba::parse("RGB(255, 136, 0, 0.5)"),
        Ok(Rgba::new(255, 136, 0, 128))
    );
    assert_eq!(
        Rgba::parse("rgba(255, 136, 0, 1)"),
        Ok(Rgba::new(255, 136, 0, 255))
    );
    assert_eq!(
        Rgba::parse("rgba(255 136 0 / 50%)"),
        Ok(Rgba::new(255, 136, 0, 128))
    );
    assert_eq!(Rgba::parse("rgb(0 0 0 / 0)"), Ok(Rgba::new(0, 0, 0, 0)));
    assert_eq!(Rgba::parse("rgb(0 0 0 /0.2)"), Ok(Rgba::new(0, 0, 0, 51)));
}

#[test]
fn clamps_an_alpha_outside_the_unit_range_like_css() {
    assert_eq!(Rgba::parse("rgb(0 0 0 / 2)"), Ok(Rgba::new(0, 0, 0, 255)));
    assert_eq!(Rgba::parse("rgb(0 0 0 / -1)"), Ok(Rgba::new(0, 0, 0, 0)));
    assert_eq!(
        Rgba::parse("rgb(0 0 0 / 150%)"),
        Ok(Rgba::new(0, 0, 0, 255))
    );
}

#[test]
fn rejects_malformed_functions() {
    for text in [
        "rgb(1 2 / 0.5)",
        "rgb(1 2 3 4 / 0.5)",
        "rgb(1 2 3 / )",
        "rgb(1 2 3 / abc)",
        "rgb(1 2 3 / nan)",
        "rgb(1 2 3 / inf)",
        "rgb(1, 2, 3, x)",
        "rgb(a b c / 1)",
        "rgb(1 2 3 4 5)",
    ] {
        assert_eq!(
            Rgba::parse(text),
            Err(ParseColorError::InvalidFunction),
            "{text}"
        );
    }
    assert_eq!(
        Rgba::parse("rgba(256 0 0 / 1)"),
        Err(ParseColorError::OutOfRange)
    );
}

#[test]
fn reports_a_bad_channel_whichever_it_is() {
    for text in [
        "rgb(256 0 0 / 1)",
        "rgb(0 256 0 / 1)",
        "rgb(0 0 256 / 1)",
        "rgba(0, 0, 999, 1)",
    ] {
        assert_eq!(
            Rgba::parse(text),
            Err(ParseColorError::OutOfRange),
            "{text}"
        );
    }
    for text in [
        "rgb(x 0 0 / 1)",
        "rgb(0 x 0 / 1)",
        "rgb(0 0 x / 1)",
        "rgb(1.5 0 0 / 1)",
        "rgb(-1 0 0 / 1)",
    ] {
        assert_eq!(
            Rgba::parse(text),
            Err(ParseColorError::InvalidFunction),
            "{text}"
        );
    }
}

#[test]
fn passes_the_errors_of_rgb_through() {
    assert_eq!(Rgba::parse(""), Err(ParseColorError::Empty));
    assert_eq!(Rgba::parse("nope"), Err(ParseColorError::UnknownName));
    assert_eq!(
        Rgba::parse("#12345"),
        Err(ParseColorError::InvalidLength { length: 5 })
    );
}

#[test]
fn writes_hex_and_css() {
    assert_eq!(Rgba::new(255, 136, 0, 255).to_hex(), "#ff8800");
    assert_eq!(Rgba::new(255, 136, 0, 128).to_hex(), "#ff880080");
    assert_eq!(Rgba::new(255, 136, 0, 255).to_css(), "rgb(255 136 0)");
    assert_eq!(
        Rgba::new(255, 136, 0, 128).to_css(),
        "rgb(255 136 0 / 0.502)"
    );
    assert_eq!(Rgba::new(255, 136, 0, 0).to_css(), "rgb(255 136 0 / 0)");
    assert_eq!(Rgba::new(255, 136, 0, 128).to_string(), "#ff880080");
}

#[test]
fn from_str_is_parse() {
    assert_eq!("#ff880080".parse::<Rgba>(), Ok(Rgba::new(255, 136, 0, 128)));
    assert!("nope".parse::<Rgba>().is_err());
}
