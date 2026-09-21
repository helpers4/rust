// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn prefixes_every_line() {
    assert_eq!(indent("a\nb", "  "), "  a\n  b");
}

#[test]
fn leaves_blank_lines_untouched() {
    assert_eq!(indent("a\n\n  \nb", "> "), "> a\n\n  \n> b");
}

#[test]
fn keeps_a_trailing_newline() {
    assert_eq!(indent("a\n", "  "), "  a\n");
}

#[test]
fn empty_input_stays_empty() {
    assert_eq!(indent("", "  "), "");
}

#[test]
fn empty_prefix_changes_nothing() {
    assert_eq!(indent("a\nb", ""), "a\nb");
}
