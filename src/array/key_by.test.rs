// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn indexes_by_key() {
    let indexed = key_by(&["apple", "bob", "cherry"], |s| s.len());
    assert_eq!(indexed[&5], "apple");
    assert_eq!(indexed[&3], "bob");
    assert_eq!(indexed[&6], "cherry");
    assert_eq!(indexed.len(), 3);
}

#[test]
fn the_last_element_wins_on_a_shared_key() {
    let indexed = key_by(&[("a", 1), ("b", 1)], |&(_, k)| k);
    assert_eq!(indexed[&1], ("b", 1));
}

#[test]
fn empty_input_gives_empty_map() {
    assert!(key_by(&[] as &[i32], |x| *x).is_empty());
}
