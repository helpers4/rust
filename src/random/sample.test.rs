// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn picks_the_requested_number_of_different_positions() {
    let items: Vec<u32> = (0..20).collect();
    let picked = sample(&items, 5, &mut Rng::seeded(4));
    assert_eq!(picked.len(), 5);
    let mut sorted = picked.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 5);
}

#[test]
fn a_count_of_zero_gives_nothing() {
    assert_eq!(
        sample(&[1, 2, 3], 0, &mut Rng::seeded(1)),
        Vec::<i32>::new()
    );
    assert_eq!(
        sample::<i32>(&[], 3, &mut Rng::seeded(1)),
        Vec::<i32>::new()
    );
}

#[test]
fn a_large_count_returns_everything_shuffled() {
    let mut all = sample(&[1, 2, 3, 4], 99, &mut Rng::seeded(2));
    all.sort_unstable();
    assert_eq!(all, [1, 2, 3, 4]);
}

#[test]
fn equal_values_can_both_be_picked() {
    assert_eq!(sample(&[7, 7], 2, &mut Rng::seeded(5)), vec![7, 7]);
}

#[test]
fn is_reproducible_and_leaves_the_input_alone() {
    let items = [10, 20, 30, 40, 50];
    let first = sample(&items, 3, &mut Rng::seeded(8));
    assert_eq!(first, sample(&items, 3, &mut Rng::seeded(8)));
    assert_eq!(items, [10, 20, 30, 40, 50]);
}

#[test]
fn every_element_is_picked_about_equally_often() {
    let mut rng = Rng::seeded(31);
    let mut counts = [0u32; 5];
    for _ in 0..10_000 {
        for picked in sample(&[0usize, 1, 2, 3, 4], 2, &mut rng) {
            counts[picked] += 1;
        }
    }
    for count in counts {
        assert!((3_700..=4_300).contains(&count), "{counts:?}");
    }
}
