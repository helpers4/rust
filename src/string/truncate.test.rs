// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn returns_input_when_it_fits() {
    assert_eq!(truncate("abc", 3, "..."), "abc");
    assert_eq!(truncate("", 0, "..."), "");
}

#[test]
fn cuts_and_appends_suffix_within_the_limit() {
    assert_eq!(truncate("Hello, world", 8, "..."), "Hello...");
    assert_eq!(truncate("abcdef", 4, ""), "abcd");
}

#[test]
fn suffix_longer_than_limit_is_itself_truncated() {
    assert_eq!(truncate("Hello", 2, "..."), "..");
    assert_eq!(truncate("Hello", 3, "..."), "...");
    assert_eq!(truncate("Hello", 0, "..."), "");
}

#[test]
fn counts_chars_not_bytes() {
    assert_eq!(truncate("éééé", 3, "…"), "éé…");
}
