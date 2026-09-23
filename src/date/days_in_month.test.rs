// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn long_months_have_31_days() {
    for month in [1, 3, 5, 7, 8, 10, 12] {
        assert_eq!(days_in_month(2023, month), Some(31), "month {month}");
    }
}

#[test]
fn short_months_have_30_days() {
    for month in [4, 6, 9, 11] {
        assert_eq!(days_in_month(2023, month), Some(30), "month {month}");
    }
}

#[test]
fn february_depends_on_the_year() {
    assert_eq!(days_in_month(2024, 2), Some(29));
    assert_eq!(days_in_month(2023, 2), Some(28));
    assert_eq!(days_in_month(1900, 2), Some(28));
    assert_eq!(days_in_month(2000, 2), Some(29));
}

#[test]
fn an_invalid_month_gives_none() {
    assert_eq!(days_in_month(2023, 0), None);
    assert_eq!(days_in_month(2023, 13), None);
}
