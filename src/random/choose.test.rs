// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn an_empty_slice_has_nothing_to_choose() {
    assert_eq!(choose::<i32>(&[], &mut Rng::seeded(1)), None);
}

#[test]
fn a_single_element_is_always_chosen() {
    assert_eq!(choose(&["only"], &mut Rng::seeded(1)), Some(&"only"));
}

#[test]
fn every_element_is_chosen_about_equally_often() {
    let mut rng = Rng::seeded(77);
    let items = [0usize, 1, 2, 3];
    let mut counts = [0u32; 4];
    for _ in 0..8_000 {
        if let Some(&picked) = choose(&items, &mut rng) {
            counts[picked] += 1;
        }
    }
    for count in counts {
        assert!((1_800..=2_200).contains(&count), "{counts:?}");
    }
}
