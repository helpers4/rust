// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_the_listed_keys() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(omit(&map, &["b"]), HashMap::from([("a", 1), ("c", 3)]));
}

#[test]
fn ignores_keys_that_are_missing() {
    let map = HashMap::from([("a", 1)]);
    assert_eq!(omit(&map, &["z"]), HashMap::from([("a", 1)]));
}

#[test]
fn no_keys_returns_a_copy() {
    let map = HashMap::from([("a", 1), ("b", 2)]);
    assert_eq!(omit(&map, &[]), map);
}
