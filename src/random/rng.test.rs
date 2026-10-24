// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn matches_the_splitmix64_reference_vectors() {
    let mut rng = Rng::seeded(0);
    assert_eq!(rng.next_u64(), 0xE220_A839_7B1D_CDAF);
    assert_eq!(rng.next_u64(), 0x6E78_9E6A_A1B9_65F4);
    assert_eq!(rng.next_u64(), 0x06C4_5D18_8009_454F);
    let mut rng = Rng::seeded(0xDEAD_BEEF);
    assert_eq!(rng.next_u64(), 0x4ADF_B90F_68C9_EB9B);
    assert_eq!(rng.next_u64(), 0xDE58_6A31_41A1_0922);
}

#[test]
fn equal_seeds_give_equal_sequences_and_clones_continue_alike() {
    let mut a = Rng::seeded(5);
    let mut b = Rng::seeded(5);
    let mut c = a.clone();
    for _ in 0..10 {
        let x = a.next_u64();
        assert_eq!(x, b.next_u64());
        assert_eq!(x, c.next_u64());
    }
    assert_eq!(a, b);
    assert_ne!(Rng::seeded(5).next_u64(), Rng::seeded(6).next_u64());
}

#[test]
fn next_f64_is_the_top_53_bits_of_the_next_draw_over_two_to_the_53() {
    // Reference values: (u64 >> 11) / 2^53 for the SplitMix64 draws of seeds 0 and 1.
    let mut rng = Rng::seeded(0);
    assert_eq!(
        [rng.next_f64(), rng.next_f64()],
        [0.883_310_808_213_642_6, 0.431_527_997_048_509_97]
    );
    let mut rng = Rng::seeded(1);
    assert_eq!(
        [rng.next_f64(), rng.next_f64()],
        [0.566_561_575_172_280_9, 0.745_781_757_262_701_1]
    );
}

#[test]
fn next_f64_has_a_mean_near_one_half() {
    let mut rng = Rng::seeded(5);
    let mean: f64 = (0..10_000).map(|_| rng.next_f64()).sum::<f64>() / 10_000.0;
    assert!((mean - 0.5).abs() < 0.02, "{mean}");
}

#[test]
fn next_f64_stays_in_the_unit_interval() {
    let mut rng = Rng::seeded(11);
    for _ in 0..1000 {
        let x = rng.next_f64();
        assert!((0.0..1.0).contains(&x));
    }
}

#[test]
fn below_zero_is_none_and_below_one_is_zero() {
    let mut rng = Rng::seeded(1);
    let before = rng.clone();
    assert_eq!(rng.below(0), None);
    assert_eq!(rng, before); // nothing was drawn
    assert_eq!(rng.below(1), Some(0));
}

#[test]
fn below_covers_every_value_roughly_evenly() {
    let mut rng = Rng::seeded(2024);
    let mut counts = [0u32; 6];
    for _ in 0..60_000 {
        if let Some(n) = rng.below(6).and_then(|n| usize::try_from(n).ok()) {
            counts[n] += 1;
        }
    }
    for count in counts {
        assert!((9_000..=11_000).contains(&count), "{counts:?}");
    }
}

#[test]
fn below_rejects_draws_in_the_biased_zone() {
    // With a bound just above 2^63, about half of the draws fall in the biased zone and are
    // redrawn: some seed must consume more than one 64-bit draw.
    let bound = (1u64 << 63) + 1;
    let redrew = (0..200u64).any(|seed| {
        let mut rng = Rng::seeded(seed);
        let result = rng.below(bound);
        let mut single = Rng::seeded(seed);
        single.next_u64();
        assert!(result.is_some_and(|n| n < bound));
        rng != single
    });
    assert!(redrew);
}

#[test]
fn below_the_largest_bound_stays_in_range() {
    let mut rng = Rng::seeded(8);
    for _ in 0..100 {
        assert!(rng.below(u64::MAX).is_some_and(|n| n < u64::MAX));
    }
}

#[test]
fn fill_bytes_writes_the_little_endian_bytes_of_each_draw() {
    for len in 0..=17usize {
        let mut buf = vec![0u8; len];
        Rng::seeded(4).fill_bytes(&mut buf);
        let mut reference = Rng::seeded(4);
        let expected: Vec<u8> = (0..len.div_ceil(8))
            .flat_map(|_| reference.next_u64().to_le_bytes())
            .take(len)
            .collect();
        assert_eq!(buf, expected, "len {len}");
    }
}

#[test]
fn index_is_zero_for_an_empty_range_and_in_range_otherwise() {
    let mut rng = Rng::seeded(6);
    assert_eq!(rng.index(0), 0);
    assert_eq!(rng.index(1), 0);
    for _ in 0..100 {
        assert!(rng.index(7) < 7);
    }
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
#[test]
fn from_entropy_gives_a_different_generator_each_time() {
    let a = Rng::from_entropy();
    let b = Rng::from_entropy();
    assert!(a.is_ok() && b.is_ok());
    assert_ne!(a, b); // equal with probability 2^-64
}

/// Rejection sampling written the plain way: redraw until the low 64 bits of the 128-bit
/// product reach the threshold below which the draws would be biased.
fn reference_below(rng: &mut Rng, bound: u64) -> u64 {
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let product = u128::from(rng.next_u64()) * u128::from(bound);
        if u64::try_from(product & u128::from(u64::MAX)).unwrap_or(0) >= threshold {
            return u64::try_from(product >> 64).unwrap_or(0);
        }
    }
}

#[test]
fn below_agrees_with_plain_rejection_sampling_including_the_redrawing_branch() {
    // 2^63 + 1 redraws about half of the time, 3 and 1000 almost never, 2^64 - 1 hardly ever.
    for bound in [3, 6, 1000, (1u64 << 63) + 1, (1u64 << 62) + 12345, u64::MAX] {
        for seed in 0..300 {
            let mut fast = Rng::seeded(seed);
            let mut plain = Rng::seeded(seed);
            assert_eq!(
                fast.below(bound),
                Some(reference_below(&mut plain, bound)),
                "bound {bound} seed {seed}"
            );
            assert_eq!(
                fast, plain,
                "the same number of draws: bound {bound} seed {seed}"
            );
        }
    }
}
