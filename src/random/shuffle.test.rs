// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn empty_and_single_inputs_are_returned_as_they_are() {
    let mut rng = Rng::seeded(1);
    assert_eq!(shuffle::<i32>(&[], &mut rng), Vec::<i32>::new());
    assert_eq!(shuffle(&[9], &mut rng), vec![9]);
}

#[test]
fn keeps_the_same_elements() {
    let items: Vec<u32> = (0..50).collect();
    let mut out = shuffle(&items, &mut Rng::seeded(3));
    assert_ne!(out, items); // 1 in 50! chance of failing
    out.sort_unstable();
    assert_eq!(out, items);
}

#[test]
fn does_not_touch_the_input_and_is_reproducible() {
    let items = ["a", "b", "c", "d"];
    let first = shuffle(&items, &mut Rng::seeded(12));
    assert_eq!(items, ["a", "b", "c", "d"]);
    assert_eq!(first, shuffle(&items, &mut Rng::seeded(12)));
}

#[test]
fn every_permutation_of_three_shows_up_about_equally() {
    let mut rng = Rng::seeded(99);
    let mut counts = std::collections::HashMap::new();
    for _ in 0..6_000 {
        *counts.entry(shuffle(&[1, 2, 3], &mut rng)).or_insert(0u32) += 1;
    }
    assert_eq!(counts.len(), 6);
    for count in counts.values() {
        assert!((800..=1200).contains(count), "{counts:?}");
    }
}
