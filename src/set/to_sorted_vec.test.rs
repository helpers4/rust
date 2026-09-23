// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn lists_the_elements_in_ascending_order() {
    assert_eq!(to_sorted_vec(&HashSet::from([3, 1, 2])), vec![1, 2, 3]);
}

#[test]
fn works_on_strings() {
    let set = HashSet::from(["pear".to_string(), "apple".to_string()]);
    assert_eq!(
        to_sorted_vec(&set),
        vec!["apple".to_string(), "pear".to_string()]
    );
}

#[test]
fn empty_gives_empty() {
    assert!(to_sorted_vec(&HashSet::<i32>::new()).is_empty());
}
