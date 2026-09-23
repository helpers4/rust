// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const BASE: Duration = Duration::from_millis(100);
const MAX: Duration = Duration::from_secs(1);

#[test]
fn doubles_with_each_attempt() {
    assert_eq!(backoff(1, BASE, MAX), Duration::from_millis(100));
    assert_eq!(backoff(2, BASE, MAX), Duration::from_millis(200));
    assert_eq!(backoff(3, BASE, MAX), Duration::from_millis(400));
    assert_eq!(backoff(4, BASE, MAX), Duration::from_millis(800));
}

#[test]
fn never_exceeds_the_maximum() {
    assert_eq!(backoff(5, BASE, MAX), MAX);
    assert_eq!(backoff(31, BASE, MAX), MAX);
    assert_eq!(backoff(32, BASE, MAX), MAX);
    assert_eq!(backoff(u32::MAX, BASE, MAX), MAX);
}

#[test]
fn attempt_zero_is_like_attempt_one() {
    assert_eq!(backoff(0, BASE, MAX), BASE);
}

#[test]
fn a_base_above_the_maximum_is_capped() {
    assert_eq!(backoff(1, Duration::from_secs(5), MAX), MAX);
}

#[test]
fn a_huge_base_saturates_instead_of_overflowing() {
    assert_eq!(backoff(40, Duration::MAX, Duration::MAX), Duration::MAX);
}
