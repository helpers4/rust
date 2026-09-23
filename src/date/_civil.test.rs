// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn the_epoch_is_day_zero() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    assert_eq!(civil_from_days(0), (1970, 1, 1));
}

#[test]
fn known_dates() {
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    assert_eq!(civil_from_days(11_017), (2000, 3, 1));
    assert_eq!(days_from_civil(1969, 12, 31), -1);
    assert_eq!(days_from_civil(2038, 1, 19), 24_855);
}

#[test]
fn the_ends_of_the_supported_range() {
    assert_eq!(days_from_civil(0, 1, 1), -719_528);
    assert_eq!(days_from_civil(9999, 12, 31), 2_932_896);
    assert_eq!(civil_from_days(-719_528), (0, 1, 1));
    assert_eq!(civil_from_days(2_932_896), (9999, 12, 31));
}

#[test]
fn leap_days_are_counted() {
    assert_eq!(
        days_from_civil(2024, 3, 1) - days_from_civil(2024, 2, 28),
        2
    );
    assert_eq!(
        days_from_civil(2023, 3, 1) - days_from_civil(2023, 2, 28),
        1
    );
    assert_eq!(
        days_from_civil(2100, 3, 1) - days_from_civil(2100, 2, 28),
        1
    );
    assert_eq!(
        days_from_civil(2000, 3, 1) - days_from_civil(2000, 2, 28),
        2
    );
}
