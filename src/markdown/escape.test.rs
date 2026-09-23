// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn escapes_inline_syntax() {
    assert_eq!(
        escape("*a* _b_ `c` [d](e) <f> g|h ~i~ j&k"),
        "\\*a\\* \\_b\\_ \\`c\\` \\[d\\](e) \\<f\\> g\\|h \\~i\\~ j\\&k"
    );
}

#[test]
fn escapes_the_backslash_itself() {
    assert_eq!(escape("a\\b"), "a\\\\b");
}

#[test]
fn escapes_block_syntax_at_the_start_of_a_line() {
    assert_eq!(escape("# title"), "\\# title");
    assert_eq!(escape("+ item"), "\\+ item");
    assert_eq!(escape("- item"), "\\- item");
    assert_eq!(escape("=== underline"), "\\=== underline");
    assert_eq!(escape("> quote"), "\\> quote");
    assert_eq!(escape("1. item"), "1\\. item");
    assert_eq!(escape("12) item"), "12\\) item");
}

#[test]
fn leaves_block_characters_alone_in_the_middle_of_a_line() {
    assert_eq!(escape("a # b + c - d = e 1. f"), "a # b + c - d = e 1. f");
}

#[test]
fn keeps_the_indentation() {
    assert_eq!(escape("  - item"), "  \\- item");
    assert_eq!(escape("   "), "   ");
}

#[test]
fn a_number_that_is_not_a_list_marker_is_left_alone() {
    assert_eq!(escape("2026 was a year"), "2026 was a year");
    assert_eq!(escape("3"), "3");
    assert_eq!(escape("3x"), "3x");
}

#[test]
fn handles_every_line_of_a_multiline_text() {
    assert_eq!(escape("# a\n- b\n\nc *d*\n"), "\\# a\n\\- b\n\nc \\*d\\*\n");
}

#[test]
fn plain_text_and_empty_text_are_unchanged() {
    assert_eq!(
        escape("plain text, with punctuation!"),
        "plain text, with punctuation!"
    );
    assert_eq!(escape(""), "");
}
