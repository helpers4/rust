// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        DateError::InvalidYear { year: 10_000 }.to_string(),
        "year 10000 is outside 0..=9999"
    );
    assert_eq!(
        DateError::InvalidMonth { month: 13 }.to_string(),
        "month 13 is not in 1..=12"
    );
    assert_eq!(
        DateError::InvalidDay { day: 31, max: 30 }.to_string(),
        "day 31 is not in 1..=30"
    );
    assert_eq!(
        DateError::InvalidFormat.to_string(),
        "expected a date written YYYY-MM-DD"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(DateError::InvalidFormat);
    assert!(err.source().is_none());
}
