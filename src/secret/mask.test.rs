// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_the_last_characters() {
    assert_eq!(mask("sk-abcdef1234567890", 4), "***************7890");
}

#[test]
fn never_shows_more_than_a_quarter() {
    assert_eq!(mask("abcdefgh", 8), "******gh");
    assert_eq!(mask("abcdefgh", 2), "******gh");
    assert_eq!(mask("abcdefgh", 1), "*******h");
    assert_eq!(mask("abcdefghijkl", 100), "*********jkl");
}

#[test]
fn short_secrets_show_at_most_a_quarter() {
    assert_eq!(mask("abc", 4), "***");
    assert_eq!(mask("abcd", 4), "***d");
    assert_eq!(mask("a", 1), "*");
}

#[test]
fn zero_visible_masks_everything() {
    assert_eq!(mask("abcdefgh", 0), "********");
}

#[test]
fn empty_stays_empty() {
    assert_eq!(mask("", 4), "");
}

#[test]
fn counts_characters_not_bytes() {
    assert_eq!(mask("éééééééé", 2), "******éé");
}
