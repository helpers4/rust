// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn matches_when_any_pattern_matches() {
    let set = GlobSet::new(&["*.fig", "frame-?"]);
    assert!(set.is_match("a.fig"));
    assert!(set.is_match("frame-1"));
    assert!(!set.is_match("a.sketch"));
}

#[test]
fn an_exclusion_wins_whatever_its_position() {
    for patterns in [["!draft-*", "*.fig"], ["*.fig", "!draft-*"]] {
        let set = GlobSet::new(&patterns);
        assert!(set.is_match("a.fig"));
        assert!(!set.is_match("draft-a.fig"));
    }
}

#[test]
fn with_only_exclusions_everything_else_matches() {
    let set = GlobSet::new(&["!*.tmp"]);
    assert!(set.is_match("a.fig"));
    assert!(!set.is_match("a.tmp"));
}

#[test]
fn an_empty_set_matches_everything() {
    assert!(GlobSet::new(&[]).is_match("anything"));
    assert!(GlobSet::new(&[]).is_match(""));
    assert_eq!(GlobSet::new(&[]), GlobSet::default());
}

#[test]
fn a_backslash_makes_a_leading_bang_literal() {
    let set = GlobSet::new(&["\\!important"]);
    assert!(set.is_match("!important"));
    assert!(!set.is_match("important"));
}

#[test]
fn a_backslash_before_anything_else_is_just_a_character() {
    let set = GlobSet::new(&["\\a"]);
    assert!(set.is_match("\\a"));
    assert!(!set.is_match("a"));
}

#[test]
fn is_case_sensitive() {
    assert!(!GlobSet::new(&["Screen *"]).is_match("screen 2"));
}
