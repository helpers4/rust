// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn years_divisible_by_four_are_leap() {
    assert!(is_leap_year(2024));
    assert!(is_leap_year(1996));
}

#[test]
fn centuries_are_not_leap_unless_divisible_by_400() {
    assert!(!is_leap_year(1900));
    assert!(!is_leap_year(2100));
    assert!(is_leap_year(2000));
    assert!(is_leap_year(1600));
}

#[test]
fn other_years_are_not_leap() {
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(2026));
}

#[test]
fn works_for_year_zero_and_negative_years() {
    assert!(is_leap_year(0));
    assert!(is_leap_year(-4));
    assert!(!is_leap_year(-1));
}
