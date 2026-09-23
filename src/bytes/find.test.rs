// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn finds_the_first_match() {
    assert_eq!(find(b"abcabc", b"bc"), Some(1));
}

#[test]
fn finds_a_match_at_the_very_end() {
    assert_eq!(find(b"abcd", b"cd"), Some(2));
}

#[test]
fn no_match_gives_none() {
    assert_eq!(find(b"abc", b"d"), None);
}

#[test]
fn a_needle_longer_than_the_haystack_gives_none() {
    assert_eq!(find(b"ab", b"abc"), None);
}

#[test]
fn an_empty_needle_matches_at_the_start() {
    assert_eq!(find(b"abc", b""), Some(0));
    assert_eq!(find(b"", b""), Some(0));
}
