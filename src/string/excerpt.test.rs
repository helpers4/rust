// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn text_already_within_the_limit_is_unchanged() {
    assert_eq!(
        excerpt("A short game about ducks.", 200, "…"),
        "A short game about ducks."
    );
}

#[test]
fn cuts_at_the_end_of_the_last_sentence_that_fits() {
    assert_eq!(
        excerpt(
            "Build the biggest, best theme park ever seen. Can you make money?",
            47,
            "…"
        ),
        "Build the biggest, best theme park ever seen."
    );
}

#[test]
fn falls_back_to_a_word_boundary_when_no_sentence_fits() {
    assert_eq!(
        excerpt(
            "This description has no punctuation at all so it must cut on a word",
            30,
            "…"
        ),
        "This description has no…"
    );
}

#[test]
fn falls_back_to_truncate_for_one_giant_unbroken_word() {
    let long_word = "a".repeat(50);
    assert_eq!(excerpt(&long_word, 10, "…"), format!("{}…", "a".repeat(9)));
}

#[test]
fn collapses_internal_whitespace_first() {
    assert_eq!(excerpt("hello   \n  world", 200, "…"), "hello world");
}

#[test]
fn a_sentence_end_right_at_the_window_edge_with_no_trailing_whitespace_counts() {
    // "abc." fills the whole 4-char window with no character after the '.': the end-of-window
    // case of the "followed by whitespace or the end" rule.
    assert_eq!(excerpt("abc.", 4, "…"), "abc.");
}

#[test]
fn leading_and_trailing_whitespace_is_trimmed_before_measuring() {
    assert_eq!(excerpt("   hello world   ", 200, "…"), "hello world");
}
