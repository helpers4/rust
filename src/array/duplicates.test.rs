// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn reports_repeated_elements_once_in_first_seen_order() {
    assert_eq!(duplicates(&[3, 1, 3, 2, 1, 3]), vec![3, 1]);
}

#[test]
fn no_duplicates_gives_empty() {
    assert_eq!(duplicates(&[1, 2, 3]), Vec::<i32>::new());
}

#[test]
fn empty_input_gives_empty() {
    assert_eq!(duplicates(&[] as &[i32]), Vec::<i32>::new());
}

#[test]
fn works_with_owned_strings() {
    let items = ["a".to_string(), "b".to_string(), "a".to_string()];
    assert_eq!(duplicates(&items), vec!["a".to_string()]);
}
