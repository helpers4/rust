// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn transforms_every_value() {
    let map = HashMap::from([("a", 1), ("b", 2)]);
    assert_eq!(
        map_values(&map, |v| v * 10),
        HashMap::from([("a", 10), ("b", 20)])
    );
}

#[test]
fn can_change_the_value_type() {
    let map = HashMap::from([("a", 1)]);
    assert_eq!(
        map_values(&map, ToString::to_string),
        HashMap::from([("a", "1".to_string())])
    );
}

#[test]
fn empty_stays_empty() {
    assert!(map_values(&HashMap::<u8, u8>::new(), |v| *v).is_empty());
}
