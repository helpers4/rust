// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn prefixes_every_line() {
    assert_eq!(blockquote("a\nb"), "> a\n> b");
}

#[test]
fn empty_lines_get_a_bare_marker() {
    assert_eq!(blockquote("a\n\nb"), "> a\n>\n> b");
}

#[test]
fn keeps_a_trailing_newline() {
    assert_eq!(blockquote("a\n"), "> a\n>");
}

#[test]
fn nests() {
    assert_eq!(blockquote(&blockquote("a")), "> > a");
}

#[test]
fn empty_text_is_a_bare_marker() {
    assert_eq!(blockquote(""), ">");
}
