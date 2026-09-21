// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn formats_each_unit() {
    assert_eq!(format(Duration::from_millis(250)), "250ms");
    assert_eq!(format(Duration::from_secs(45)), "45s");
    assert_eq!(format(Duration::from_secs(120)), "2m");
    assert_eq!(format(Duration::from_secs(3600)), "1h");
    assert_eq!(format(Duration::from_secs(86_400)), "1d");
}

#[test]
fn combines_units_largest_first_and_skips_zeros() {
    assert_eq!(format(Duration::from_secs(5405)), "1h 30m 5s");
    assert_eq!(format(Duration::from_secs(90_000)), "1d 1h");
    assert_eq!(format(Duration::from_millis(1500)), "1s 500ms");
    assert_eq!(format(Duration::from_secs(3601)), "1h 1s");
}

#[test]
fn zero_is_zero_seconds() {
    assert_eq!(format(Duration::ZERO), "0s");
}

#[test]
fn drops_what_is_below_a_millisecond() {
    assert_eq!(format(Duration::from_micros(999)), "0s");
    assert_eq!(format(Duration::from_micros(1999)), "1ms");
}

#[test]
fn days_are_the_largest_unit() {
    assert_eq!(format(Duration::from_secs(90 * 86_400)), "90d");
}
