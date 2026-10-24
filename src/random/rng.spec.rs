// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;
use proptest::prelude::*;

proptest! {
    #[test]
    fn below_is_always_under_the_bound(seed in any::<u64>(), bound in 1u64..) {
        prop_assert!(Rng::seeded(seed).below(bound).is_some_and(|n| n < bound));
    }

    #[test]
    fn the_sequence_depends_only_on_the_seed(seed in any::<u64>(), n in 1usize..40) {
        let mut a = Rng::seeded(seed);
        let mut b = Rng::seeded(seed);
        let xs: Vec<u64> = (0..n).map(|_| a.next_u64()).collect();
        let ys: Vec<u64> = (0..n).map(|_| b.next_u64()).collect();
        prop_assert_eq!(xs, ys);
    }

    #[test]
    fn next_f64_is_in_the_unit_interval(seed in any::<u64>()) {
        let x = Rng::seeded(seed).next_f64();
        prop_assert!((0.0..1.0).contains(&x));
    }

    #[test]
    fn fill_bytes_is_a_prefix_of_a_longer_fill(seed in any::<u64>(), short in 0usize..40, extra in 0usize..40) {
        let mut a = vec![0u8; short];
        let mut b = vec![0u8; short + extra];
        Rng::seeded(seed).fill_bytes(&mut a);
        Rng::seeded(seed).fill_bytes(&mut b);
        prop_assert_eq!(&b[..short], &a[..]);
    }
}
