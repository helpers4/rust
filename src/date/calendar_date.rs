// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_civil::{civil_from_days, days_from_civil};
use super::{DateError, Weekday, days_in_month, is_leap_year};
use std::fmt;
use std::str::FromStr;

/// The lowest day count a [`Date`] can hold: 0000-01-01.
const MIN_DAYS: i64 = -719_528;
/// The highest day count a [`Date`] can hold: 9999-12-31.
const MAX_DAYS: i64 = 2_932_896;

/// A calendar date (year, month, day) in the proleptic Gregorian calendar, with no time of day
/// and no time zone.
///
/// The year is limited to `0..=9999`, so the ISO 8601 form `YYYY-MM-DD` always has four year
/// digits. Dates compare chronologically. Being a plain calendar date, it cannot express "now":
/// to get today's date, convert a unix time with [`Date::from_unix_days`] (the current time is
/// an input, never read behind your back).
///
/// # Examples
///
/// ```
/// use helpers4::date::Date;
///
/// let date = Date::new(2026, 9, 23)?;
/// assert_eq!(date.to_string(), "2026-09-23");
/// assert_eq!(date.add_days(10), Some(Date::new(2026, 10, 3)?));
/// assert_eq!("2024-02-29".parse::<Date>(), Date::new(2024, 2, 29));
/// # Ok::<(), helpers4::date::DateError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// Builds a date, checking that it exists.
    ///
    /// # Arguments
    ///
    /// - `year` - The year, `0..=9999`.
    /// - `month` - The month, `1..=12`.
    /// - `day` - The day of the month, `1` to the length of that month.
    ///
    /// # Errors
    ///
    /// [`DateError::InvalidYear`], [`DateError::InvalidMonth`] or [`DateError::InvalidDay`] when
    /// the date does not exist (such as February 30th).
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, DateError> {
        if !(0..=9999).contains(&year) {
            return Err(DateError::InvalidYear { year });
        }
        let max = days_in_month(year, month).ok_or(DateError::InvalidMonth { month })?;
        if day == 0 || day > max {
            return Err(DateError::InvalidDay { day, max });
        }
        Ok(Self { year, month, day })
    }

    /// Parses the ISO 8601 form `YYYY-MM-DD`.
    ///
    /// Exactly four year digits, two month digits and two day digits, separated by hyphens: no
    /// spaces, signs or time of day.
    ///
    /// # Arguments
    ///
    /// - `text` - The text to parse.
    ///
    /// # Errors
    ///
    /// [`DateError::InvalidFormat`] when the text does not have that shape, or the errors of
    /// [`Date::new`] when it has the shape but names a date that does not exist.
    pub fn parse(text: &str) -> Result<Self, DateError> {
        let bytes = text.as_bytes();
        let shaped = bytes.len() == 10
            && bytes.iter().enumerate().all(|(i, b)| {
                if i == 4 || i == 7 {
                    *b == b'-'
                } else {
                    b.is_ascii_digit()
                }
            });
        if !shaped {
            return Err(DateError::InvalidFormat);
        }
        let number = |range: std::ops::Range<usize>| -> i32 {
            bytes[range]
                .iter()
                .fold(0, |n, b| n * 10 + i32::from(b - b'0'))
        };
        let (year, month, day) = (number(0..4), number(5..7), number(8..10));
        // Two digits at most, so they fit a u8.
        Self::new(
            year,
            u8::try_from(month).unwrap_or(u8::MAX),
            u8::try_from(day).unwrap_or(u8::MAX),
        )
    }

    /// The year, `0..=9999`.
    ///
    /// # Returns
    ///
    /// The year.
    #[must_use]
    pub fn year(self) -> i32 {
        self.year
    }

    /// The month, `1..=12`.
    ///
    /// # Returns
    ///
    /// The month, `1` for January.
    #[must_use]
    pub fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, starting at `1`.
    ///
    /// # Returns
    ///
    /// The day of the month.
    #[must_use]
    pub fn day(self) -> u8 {
        self.day
    }

    /// Whether the year of this date is a leap year.
    ///
    /// # Returns
    ///
    /// `true` for a date in a leap year.
    #[must_use]
    pub fn is_leap_year(self) -> bool {
        is_leap_year(self.year)
    }

    /// The day of the week.
    ///
    /// # Returns
    ///
    /// The [`Weekday`] this date falls on.
    #[must_use]
    pub fn weekday(self) -> Weekday {
        // 1970-01-01, day 0, was a Thursday: index 3 counted from Monday.
        Weekday::from_monday_index(self.unix_days() + 3)
    }

    /// The day of the year: `1` for January 1st, `365` (or `366`) for December 31st.
    ///
    /// # Returns
    ///
    /// The ordinal day, `1..=366`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 0..=365 always fits
    pub fn ordinal(self) -> u16 {
        (self.unix_days() - days_from_civil(self.year, 1, 1) + 1) as u16
    }

    /// The number of days since 1970-01-01 (negative before it).
    ///
    /// # Returns
    ///
    /// The day count, the same one [`Date::from_unix_days`] reads back.
    #[must_use]
    pub fn unix_days(self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }

    /// The date `days` days after 1970-01-01 (before it when negative).
    ///
    /// # Arguments
    ///
    /// - `days` - The day count since 1970-01-01, for instance `unix_seconds / 86_400` (rounded
    ///   down) for a UTC date.
    ///
    /// # Returns
    ///
    /// The date, or `None` when it falls outside the years `0..=9999`.
    #[must_use]
    pub fn from_unix_days(days: i64) -> Option<Self> {
        if !(MIN_DAYS..=MAX_DAYS).contains(&days) {
            return None;
        }
        let (year, month, day) = civil_from_days(days);
        Some(Self { year, month, day })
    }

    /// The date `days` days after this one (before it when negative).
    ///
    /// # Arguments
    ///
    /// - `days` - How many days to move.
    ///
    /// # Returns
    ///
    /// The new date, or `None` when it falls outside the years `0..=9999`.
    #[must_use]
    pub fn add_days(self, days: i64) -> Option<Self> {
        Self::from_unix_days(self.unix_days().checked_add(days)?)
    }

    /// The number of days from this date to `other`.
    ///
    /// # Arguments
    ///
    /// - `other` - The date to count to.
    ///
    /// # Returns
    ///
    /// Positive when `other` is later, negative when it is earlier, `0` for the same date.
    #[must_use]
    pub fn days_until(self, other: Self) -> i64 {
        other.unix_days() - self.unix_days()
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "calendar_date.test.rs"]
mod tests;

#[cfg(test)]
#[path = "calendar_date.spec.rs"]
mod spec;
