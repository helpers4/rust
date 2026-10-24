// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn has_the_requested_number_of_characters_from_the_alphabet() {
    let alphabet = Alphabet::numbers();
    for len in [0, 1, 8, 100] {
        let id = short_id(&alphabet, len, &mut Rng::seeded(3));
        assert_eq!(id.chars().count(), len);
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }
}

#[test]
fn counts_characters_not_bytes_for_a_non_ascii_alphabet() {
    let alphabet = Alphabet::new("日本語").unwrap_or_else(|_| Alphabet::numbers());
    let id = short_id(&alphabet, 5, &mut Rng::seeded(2));
    assert_eq!(id.chars().count(), 5);
    assert!(id.chars().all(|c| "日本語".contains(c)));
}

#[test]
fn is_reproducible_and_advances_the_generator() {
    let alphabet = Alphabet::alphanumeric();
    let mut rng = Rng::seeded(9);
    let first = short_id(&alphabet, 12, &mut rng);
    let second = short_id(&alphabet, 12, &mut rng);
    assert_ne!(first, second);
    assert_eq!(first, short_id(&alphabet, 12, &mut Rng::seeded(9)));
}

#[test]
fn every_character_is_about_equally_likely() {
    let alphabet = Alphabet::new("abcde").unwrap_or_else(|_| Alphabet::numbers());
    let id = short_id(&alphabet, 25_000, &mut Rng::seeded(55));
    for symbol in "abcde".chars() {
        let count = id.matches(symbol).count();
        assert!((4_500..=5_500).contains(&count), "{symbol}: {count}");
    }
}
