// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn literal_patterns_match_only_themselves() {
    assert!(match_glob("abc", "abc"));
    assert!(!match_glob("abc", "abx"));
    assert!(!match_glob("abc", "abcd"));
    assert!(!match_glob("abcd", "abc"));
    assert!(match_glob("", ""));
    assert!(!match_glob("", "a"));
}

#[test]
fn star_matches_any_sequence() {
    assert!(match_glob("*", ""));
    assert!(match_glob("*", "anything"));
    assert!(match_glob("*.fig", "design.fig"));
    assert!(!match_glob("*.fig", "design.fig.bak"));
    assert!(match_glob("a*b*c", "a-x-b-y-c"));
    assert!(match_glob("a**b", "ab"));
}

#[test]
fn star_backtracks() {
    assert!(match_glob("*ab", "aab"));
    assert!(match_glob("a*a*a", "aaaa"));
    assert!(!match_glob("a*b", "acbc"));
}

#[test]
fn question_mark_matches_one_character() {
    assert!(match_glob("frame-?", "frame-1"));
    assert!(!match_glob("frame-?", "frame-"));
    assert!(!match_glob("frame-?", "frame-12"));
    assert!(match_glob("??", "é🎨"));
}

#[test]
fn wildcards_in_the_text_are_literal() {
    assert!(match_glob("a?c", "a*c"));
    assert!(!match_glob("a", "*"));
}

#[test]
fn is_case_sensitive() {
    assert!(!match_glob("Screen *", "screen 2"));
}
