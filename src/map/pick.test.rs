// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_only_the_requested_keys() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(pick(&map, &["a", "c"]), HashMap::from([("a", 1), ("c", 3)]));
}

#[test]
fn ignores_keys_that_are_missing() {
    let map = HashMap::from([("a", 1)]);
    assert_eq!(pick(&map, &["a", "z"]), HashMap::from([("a", 1)]));
}

#[test]
fn no_keys_gives_an_empty_map() {
    let map = HashMap::from([("a", 1)]);
    assert!(pick(&map, &[]).is_empty());
}
