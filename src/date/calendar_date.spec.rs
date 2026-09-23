// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn any_date() -> impl Strategy<Value = Date> {
    (MIN_DAYS..=MAX_DAYS).prop_map(|days| Date::from_unix_days(days).unwrap())
}

proptest! {
    #[test]
    fn unix_days_round_trip(days in MIN_DAYS..=MAX_DAYS) {
        prop_assert_eq!(Date::from_unix_days(days).unwrap().unix_days(), days);
    }

    #[test]
    fn every_day_count_gives_a_valid_date(days in MIN_DAYS..=MAX_DAYS) {
        let d = Date::from_unix_days(days).unwrap();
        prop_assert!(Date::new(d.year(), d.month(), d.day()).is_ok());
    }

    #[test]
    fn display_and_parse_round_trip(d in any_date()) {
        prop_assert_eq!(d.to_string().parse::<Date>(), Ok(d));
    }

    #[test]
    fn the_next_day_is_one_day_later_and_the_weekday_advances(d in any_date()) {
        if let Some(next) = d.add_days(1) {
            prop_assert_eq!(d.days_until(next), 1);
            prop_assert_eq!(next.weekday().iso_number() % 7, (d.weekday().iso_number() % 7 + 1) % 7);
            prop_assert!(d < next);
        }
    }

    #[test]
    fn adding_then_subtracting_gives_back_the_date(d in any_date(), n in -5000i64..5000) {
        if let Some(moved) = d.add_days(n) {
            prop_assert_eq!(moved.add_days(-n), Some(d));
            prop_assert_eq!(d.days_until(moved), n);
        }
    }

    #[test]
    fn the_ordinal_stays_within_the_year(d in any_date()) {
        let last = if d.is_leap_year() { 366 } else { 365 };
        prop_assert!((1..=last).contains(&d.ordinal()));
    }
}
