// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn extracts_the_token() {
    assert_eq!(bearer_token("Bearer abc"), Some("abc"));
    assert_eq!(
        bearer_token("Bearer abc.def-ghi_jkl~mno+pqr/stu"),
        Some("abc.def-ghi_jkl~mno+pqr/stu")
    );
    assert_eq!(bearer_token("Bearer YWJj=="), Some("YWJj=="));
}

#[test]
fn the_scheme_is_case_insensitive() {
    assert_eq!(bearer_token("bearer abc"), Some("abc"));
    assert_eq!(bearer_token("BEARER abc"), Some("abc"));
    assert_eq!(bearer_token("bEaReR abc"), Some("abc"));
}

#[test]
fn extra_spaces_between_scheme_and_token_and_around_the_value_are_ignored() {
    assert_eq!(bearer_token("Bearer    abc"), Some("abc"));
    assert_eq!(bearer_token("  Bearer abc \t"), Some("abc"));
}

#[test]
fn other_schemes_and_malformed_values_give_none() {
    assert_eq!(bearer_token("Basic dXNlcjpwYXNz"), None);
    assert_eq!(bearer_token("Bearerabc"), None);
    assert_eq!(bearer_token("abc"), None);
    assert_eq!(bearer_token(""), None);
    assert_eq!(bearer_token("Bearer"), None);
    assert_eq!(bearer_token("Bearer "), None);
}

#[test]
fn a_token_with_forbidden_characters_gives_none() {
    assert_eq!(bearer_token("Bearer a b"), None);
    assert_eq!(bearer_token("Bearer a,b"), None);
    assert_eq!(bearer_token("Bearer a\"b"), None);
    assert_eq!(bearer_token("Bearer é"), None);
    assert_eq!(bearer_token("Bearer ==="), None);
    assert_eq!(bearer_token("Bearer a=b"), None);
}
