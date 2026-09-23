// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn standard_colors_use_30_to_37_and_40_to_47() {
    let colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    for (i, color) in colors.iter().enumerate() {
        assert_eq!(color.sgr(false), (30 + i).to_string());
        assert_eq!(color.sgr(true), (40 + i).to_string());
    }
}

#[test]
fn bright_colors_use_90_to_97_and_100_to_107() {
    let colors = [
        Color::BrightBlack,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];
    for (i, color) in colors.iter().enumerate() {
        assert_eq!(color.sgr(false), (90 + i).to_string());
        assert_eq!(color.sgr(true), (100 + i).to_string());
    }
}

#[test]
fn extended_colors_use_38_or_48() {
    assert_eq!(Color::Ansi256(208).sgr(false), "38;5;208");
    assert_eq!(Color::Ansi256(208).sgr(true), "48;5;208");
    assert_eq!(Color::Rgb(1, 2, 3).sgr(false), "38;2;1;2;3");
    assert_eq!(Color::Rgb(1, 2, 3).sgr(true), "48;2;1;2;3");
}
