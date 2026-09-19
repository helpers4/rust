// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn strips_common_indent_and_outer_blank_lines() {
    assert_eq!(dedent("\n    Hello\n      World\n"), "Hello\n  World");
}

#[test]
fn works_without_outer_blank_lines() {
    assert_eq!(dedent("  a\n  b"), "a\nb");
}

#[test]
fn blank_lines_do_not_affect_the_indent() {
    assert_eq!(dedent("  a\n\n  b"), "a\n\nb");
}

#[test]
fn single_blank_line_is_kept_as_is() {
    assert_eq!(dedent(""), "");
    assert_eq!(dedent("   "), "   ");
}

#[test]
fn all_blank_lines_have_no_indent_to_strip() {
    assert_eq!(dedent("\n\n"), "");
}

#[test]
fn unindented_input_is_unchanged() {
    assert_eq!(dedent("a\nb"), "a\nb");
}

#[test]
fn counts_multibyte_whitespace_as_one_indent_unit() {
    assert_eq!(dedent("\u{a0}a\n b"), "a\nb");
}
