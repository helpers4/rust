// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn lowercases_and_drops_punctuation() {
    assert_eq!(heading_slug("Hello, World!"), "hello-world");
}

#[test]
fn turns_each_space_into_a_hyphen() {
    assert_eq!(heading_slug("a b c"), "a-b-c");
    assert_eq!(heading_slug("a  b"), "a--b");
    assert_eq!(heading_slug("a\tb"), "a-b");
}

#[test]
fn keeps_hyphens_underscores_and_digits() {
    assert_eq!(
        heading_slug("snake_case & kebab-case 2"),
        "snake_case--kebab-case-2"
    );
}

#[test]
fn trims_the_ends() {
    assert_eq!(heading_slug("  Title  "), "title");
}

#[test]
fn keeps_letters_of_any_language() {
    assert_eq!(heading_slug("Café Über"), "café-über");
    assert_eq!(heading_slug("日本語 テスト"), "日本語-テスト");
}

#[test]
fn a_letter_that_lowercases_to_several_characters_is_kept_whole() {
    assert_eq!(heading_slug("İ").chars().count(), 2);
}

#[test]
fn only_punctuation_gives_an_empty_slug() {
    assert_eq!(heading_slug("?!"), "");
    assert_eq!(heading_slug(""), "");
}
