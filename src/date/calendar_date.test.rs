// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn date(year: i32, month: u8, day: u8) -> Date {
    Date::new(year, month, day).unwrap()
}

#[test]
fn builds_valid_dates() {
    let d = date(2026, 9, 23);
    assert_eq!((d.year(), d.month(), d.day()), (2026, 9, 23));
    assert!(Date::new(2024, 2, 29).is_ok());
    assert!(Date::new(0, 1, 1).is_ok());
    assert!(Date::new(9999, 12, 31).is_ok());
}

#[test]
fn rejects_a_year_out_of_range() {
    assert_eq!(
        Date::new(-1, 1, 1),
        Err(DateError::InvalidYear { year: -1 })
    );
    assert_eq!(
        Date::new(10_000, 1, 1),
        Err(DateError::InvalidYear { year: 10_000 })
    );
}

#[test]
fn rejects_a_month_out_of_range() {
    assert_eq!(
        Date::new(2026, 0, 1),
        Err(DateError::InvalidMonth { month: 0 })
    );
    assert_eq!(
        Date::new(2026, 13, 1),
        Err(DateError::InvalidMonth { month: 13 })
    );
}

#[test]
fn rejects_a_day_that_does_not_exist() {
    assert_eq!(
        Date::new(2026, 4, 31),
        Err(DateError::InvalidDay { day: 31, max: 30 })
    );
    assert_eq!(
        Date::new(2023, 2, 29),
        Err(DateError::InvalidDay { day: 29, max: 28 })
    );
    assert_eq!(
        Date::new(2026, 1, 0),
        Err(DateError::InvalidDay { day: 0, max: 31 })
    );
}

#[test]
fn parses_iso_dates() {
    assert_eq!(Date::parse("2026-09-23"), Ok(date(2026, 9, 23)));
    assert_eq!(Date::parse("0000-01-01"), Ok(date(0, 1, 1)));
    assert_eq!("2024-02-29".parse::<Date>(), Ok(date(2024, 2, 29)));
}

#[test]
fn rejects_text_that_is_not_shaped_like_a_date() {
    for text in [
        "",
        "2026-9-23",
        "2026/09/23",
        "2026-09-2",
        "2026-09-233",
        " 2026-09-23",
        "+026-09-23",
        "abcd-ef-gh",
        "2026-09-23T00:00",
    ] {
        assert_eq!(Date::parse(text), Err(DateError::InvalidFormat), "{text:?}");
    }
}

#[test]
fn parsing_a_shaped_but_impossible_date_reports_why() {
    assert_eq!(
        Date::parse("2026-13-01"),
        Err(DateError::InvalidMonth { month: 13 })
    );
    assert_eq!(
        Date::parse("2023-02-29"),
        Err(DateError::InvalidDay { day: 29, max: 28 })
    );
    assert_eq!(
        Date::parse("2026-00-10"),
        Err(DateError::InvalidMonth { month: 0 })
    );
}

#[test]
fn displays_as_iso_with_padding() {
    assert_eq!(date(2026, 9, 3).to_string(), "2026-09-03");
    assert_eq!(date(7, 1, 1).to_string(), "0007-01-01");
}

#[test]
fn knows_the_weekday() {
    assert_eq!(date(1970, 1, 1).weekday(), Weekday::Thursday);
    assert_eq!(date(2000, 1, 1).weekday(), Weekday::Saturday);
    assert_eq!(date(2024, 1, 1).weekday(), Weekday::Monday);
    assert_eq!(date(2038, 1, 19).weekday(), Weekday::Tuesday);
    assert_eq!(date(1969, 12, 31).weekday(), Weekday::Wednesday);
}

#[test]
fn knows_the_day_of_the_year() {
    assert_eq!(date(2026, 1, 1).ordinal(), 1);
    assert_eq!(date(2026, 12, 31).ordinal(), 365);
    assert_eq!(date(2024, 12, 31).ordinal(), 366);
    assert_eq!(date(2024, 3, 1).ordinal(), 61);
}

#[test]
fn knows_if_its_year_is_a_leap_year() {
    assert!(date(2024, 6, 1).is_leap_year());
    assert!(!date(2026, 6, 1).is_leap_year());
}

#[test]
fn converts_to_and_from_unix_days() {
    assert_eq!(date(1970, 1, 1).unix_days(), 0);
    assert_eq!(Date::from_unix_days(0), Some(date(1970, 1, 1)));
    assert_eq!(Date::from_unix_days(-1), Some(date(1969, 12, 31)));
    assert_eq!(Date::from_unix_days(20_000), Some(date(2024, 10, 4)));
}

#[test]
fn from_unix_days_stops_at_the_ends_of_the_range() {
    assert_eq!(Date::from_unix_days(MIN_DAYS), Some(date(0, 1, 1)));
    assert_eq!(Date::from_unix_days(MAX_DAYS), Some(date(9999, 12, 31)));
    assert_eq!(Date::from_unix_days(MIN_DAYS - 1), None);
    assert_eq!(Date::from_unix_days(MAX_DAYS + 1), None);
}

#[test]
fn adds_days_across_months_and_years() {
    assert_eq!(date(2026, 1, 31).add_days(1), Some(date(2026, 2, 1)));
    assert_eq!(date(2024, 2, 28).add_days(2), Some(date(2024, 3, 1)));
    assert_eq!(date(2026, 12, 31).add_days(1), Some(date(2027, 1, 1)));
    assert_eq!(date(2026, 3, 1).add_days(-1), Some(date(2026, 2, 28)));
    assert_eq!(date(2026, 9, 23).add_days(0), Some(date(2026, 9, 23)));
}

#[test]
fn adding_days_out_of_range_gives_none() {
    assert_eq!(date(9999, 12, 31).add_days(1), None);
    assert_eq!(date(0, 1, 1).add_days(-1), None);
    assert_eq!(date(2026, 1, 1).add_days(i64::MAX), None);
}

#[test]
fn counts_days_between_dates() {
    assert_eq!(date(2026, 1, 1).days_until(date(2026, 1, 31)), 30);
    assert_eq!(date(2026, 1, 31).days_until(date(2026, 1, 1)), -30);
    assert_eq!(date(2026, 1, 1).days_until(date(2026, 1, 1)), 0);
    assert_eq!(date(2023, 1, 1).days_until(date(2024, 1, 1)), 365);
    assert_eq!(date(2024, 1, 1).days_until(date(2025, 1, 1)), 366);
}

#[test]
fn dates_compare_chronologically() {
    assert!(date(2025, 12, 31) < date(2026, 1, 1));
    assert!(date(2026, 1, 31) < date(2026, 2, 1));
    assert!(date(2026, 2, 1) < date(2026, 2, 2));
}
