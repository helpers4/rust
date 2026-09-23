// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn joins_encoded_pairs() {
    assert_eq!(build_query([("a", "1"), ("b", "2")]), "a=1&b=2");
}

#[test]
fn encodes_keys_and_values() {
    assert_eq!(
        build_query([("q", "rust lang"), ("k&y", "a=b")]),
        "q=rust%20lang&k%26y=a%3Db"
    );
}

#[test]
fn keeps_repeated_keys_in_order() {
    assert_eq!(build_query([("t", "a"), ("t", "b")]), "t=a&t=b");
}

#[test]
fn no_pairs_gives_an_empty_string() {
    assert_eq!(build_query(Vec::<(&str, &str)>::new()), "");
}

#[test]
fn accepts_owned_strings() {
    let pairs = vec![("a".to_string(), "b c".to_string())];
    assert_eq!(build_query(pairs), "a=b%20c");
}
