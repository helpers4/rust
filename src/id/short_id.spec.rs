// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn only_uses_characters_of_the_alphabet(seed in any::<u64>(), len in 0usize..64) {
        let alphabet = Alphabet::safe();
        let id = short_id(&alphabet, len, &mut Rng::seeded(seed));
        let allowed: Vec<char> = (0..alphabet.size()).map(|i| alphabet.pick(i)).collect();
        prop_assert_eq!(id.chars().count(), len);
        prop_assert!(id.chars().all(|c| allowed.contains(&c)));
    }
}
