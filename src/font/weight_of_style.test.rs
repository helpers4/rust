// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn reads_every_weight_name() {
    let cases = [
        ("Thin", 100.0),
        ("Hairline", 100.0),
        ("ExtraLight", 200.0),
        ("Ultra Light", 200.0),
        ("Light", 300.0),
        ("Regular", 400.0),
        ("Normal", 400.0),
        ("Book", 400.0),
        ("Roman", 400.0),
        ("Medium", 500.0),
        ("SemiBold", 600.0),
        ("Demi Bold", 600.0),
        ("Bold", 700.0),
        ("Extra Bold", 800.0),
        ("UltraBold", 800.0),
        ("Black", 900.0),
        ("Heavy", 900.0),
        ("Extra Black", 950.0),
        ("Ultra-Black", 950.0),
    ];
    for (style, weight) in cases {
        assert_eq!(weight_of_style(style), weight, "{style}");
    }
}

#[test]
fn ignores_case_separators_and_other_words() {
    assert_eq!(weight_of_style("Semi Bold Italic"), 600.0);
    assert_eq!(weight_of_style("semi_bold"), 600.0);
    assert_eq!(weight_of_style("SEMI-BOLD"), 600.0);
    assert_eq!(weight_of_style("Bold Condensed Oblique"), 700.0);
    assert_eq!(weight_of_style("Roman Bold"), 700.0);
}

#[test]
fn a_style_without_a_weight_word_is_regular() {
    assert_eq!(weight_of_style("Italic"), 400.0);
    assert_eq!(weight_of_style(""), 400.0);
    assert_eq!(weight_of_style("Condensed"), 400.0);
}
