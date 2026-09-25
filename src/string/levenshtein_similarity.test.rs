// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn identical_strings_are_fully_similar() {
    assert_eq!(levenshtein_similarity("same", "same", true), 1.0);
}

#[test]
fn both_empty_is_fully_similar() {
    assert_eq!(levenshtein_similarity("", "", true), 1.0);
}

#[test]
fn classic_example() {
    let got = levenshtein_similarity("kitten", "sitting", true);
    assert!((got - 4.0 / 7.0).abs() < 1e-9);
}

#[test]
fn case_insensitive_matches_only_when_folded() {
    assert_eq!(levenshtein_similarity("Kitten", "kitten", false), 1.0);
    assert!(levenshtein_similarity("Kitten", "kitten", true) < 1.0);
}

#[test]
fn completely_different_strings_of_equal_length_are_zero() {
    assert_eq!(levenshtein_similarity("abc", "xyz", true), 0.0);
}

#[test]
fn folding_a_character_that_expands_never_pushes_the_score_past_one() {
    // 'İ' (U+0130) lowercases to two chars ('i' + a combining dot): computing the max length
    // from the un-folded input while comparing folded strings could let this exceed 1.
    let score = levenshtein_similarity("İ", "İ", false);
    assert!((0.0..=1.0).contains(&score));
    assert_eq!(score, 1.0);
}
