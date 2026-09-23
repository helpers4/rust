// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn a_plain_style_returns_the_text_unchanged() {
    assert_eq!(Style::new().paint("text"), "text");
    assert!(Style::new().is_plain());
    assert!(Style::default().is_plain());
}

#[test]
fn each_attribute_has_its_code() {
    assert_eq!(Style::new().bold().paint("x"), "\u{1b}[1mx\u{1b}[0m");
    assert_eq!(Style::new().dim().paint("x"), "\u{1b}[2mx\u{1b}[0m");
    assert_eq!(Style::new().italic().paint("x"), "\u{1b}[3mx\u{1b}[0m");
    assert_eq!(Style::new().underline().paint("x"), "\u{1b}[4mx\u{1b}[0m");
    assert_eq!(
        Style::new().strikethrough().paint("x"),
        "\u{1b}[9mx\u{1b}[0m"
    );
}

#[test]
fn colors_set_the_foreground_and_the_background() {
    assert_eq!(
        Style::new().fg(Color::Red).paint("x"),
        "\u{1b}[31mx\u{1b}[0m"
    );
    assert_eq!(
        Style::new().bg(Color::Blue).paint("x"),
        "\u{1b}[44mx\u{1b}[0m"
    );
    assert_eq!(
        Style::new()
            .fg(Color::Rgb(1, 2, 3))
            .bg(Color::Ansi256(4))
            .paint("x"),
        "\u{1b}[38;2;1;2;3;48;5;4mx\u{1b}[0m"
    );
}

#[test]
fn everything_combines_in_a_fixed_order() {
    let style = Style::new()
        .strikethrough()
        .underline()
        .italic()
        .dim()
        .bold()
        .bg(Color::White)
        .fg(Color::Black);
    assert_eq!(style.paint("x"), "\u{1b}[1;2;3;4;9;30;47mx\u{1b}[0m");
    assert!(!style.is_plain());
}

#[test]
fn painting_empty_text_still_wraps_it() {
    assert_eq!(Style::new().bold().paint(""), "\u{1b}[1m\u{1b}[0m");
}
