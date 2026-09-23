// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn starts_full() {
    let mut bucket = TokenBucket::new(3, 1, 0);
    assert_eq!(bucket.available(0), 3);
}

#[test]
fn allows_a_burst_up_to_the_capacity_and_then_refuses() {
    let mut bucket = TokenBucket::new(2, 1, 0);
    assert!(bucket.try_acquire(0));
    assert!(bucket.try_acquire(0));
    assert!(!bucket.try_acquire(0));
}

#[test]
fn refills_over_time() {
    let mut bucket = TokenBucket::new(2, 1, 0);
    bucket.try_acquire_n(0, 2);
    assert!(!bucket.try_acquire(500)); // half a token
    assert!(bucket.try_acquire(1_000)); // a whole one
    assert!(!bucket.try_acquire(1_000));
}

#[test]
fn a_faster_refill_rate_gives_tokens_sooner() {
    let mut bucket = TokenBucket::new(5, 10, 0);
    bucket.try_acquire_n(0, 5);
    assert_eq!(bucket.available(100), 1); // 10 per second = 1 per 100 ms
    assert_eq!(bucket.available(300), 3);
}

#[test]
fn never_holds_more_than_the_capacity() {
    let mut bucket = TokenBucket::new(2, 1, 0);
    assert_eq!(bucket.available(1_000_000), 2);
}

#[test]
fn a_request_for_more_than_is_available_takes_nothing() {
    let mut bucket = TokenBucket::new(3, 1, 0);
    assert!(!bucket.try_acquire_n(0, 4));
    assert_eq!(bucket.available(0), 3);
    assert!(bucket.try_acquire_n(0, 3));
}

#[test]
fn time_going_backwards_earns_nothing() {
    let mut bucket = TokenBucket::new(1, 1, 10_000);
    assert!(bucket.try_acquire(10_000));
    assert!(!bucket.try_acquire(5_000));
    // The clock resumes from the latest time seen, not from the rewound one.
    assert!(!bucket.try_acquire(10_500));
    assert!(bucket.try_acquire(11_000));
}

#[test]
fn a_zero_rate_never_refills() {
    let mut bucket = TokenBucket::new(1, 0, 0);
    assert!(bucket.try_acquire(0));
    assert!(!bucket.try_acquire(u64::MAX));
}

#[test]
fn a_huge_elapsed_time_does_not_overflow() {
    let mut bucket = TokenBucket::new(u32::MAX, u32::MAX, 0);
    assert!(bucket.try_acquire_n(0, u32::MAX));
    assert_eq!(bucket.available(u64::MAX), u32::MAX);
}
