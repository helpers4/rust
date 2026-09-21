// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn collapses_runs_of_whitespace() {
    assert_eq!(squish("a   b"), "a b");
}

#[test]
fn trims_both_ends() {
    assert_eq!(squish("  hello world  "), "hello world");
}

#[test]
fn treats_tabs_and_line_breaks_as_whitespace() {
    assert_eq!(squish("a\n\tb\r\nc"), "a b c");
}

#[test]
fn blank_input_gives_empty() {
    assert_eq!(squish(""), "");
    assert_eq!(squish(" \n "), "");
}
