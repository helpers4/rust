// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_year_has_365_or_366_days(year in 0i32..3000) {
        let total: u32 = (1..=12).map(|m| u32::from(days_in_month(year, m).unwrap())).sum();
        prop_assert_eq!(total, if crate::date::is_leap_year(year) { 366 } else { 365 });
    }
}
