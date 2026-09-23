// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A day of the week, Monday first (ISO 8601).
///
/// # Examples
///
/// ```
/// use helpers4::date::{Date, Weekday};
///
/// let day = Date::new(2024, 1, 1)?.weekday();
/// assert_eq!(day, Weekday::Monday);
/// assert_eq!(day.name(), "Monday");
/// assert_eq!(day.iso_number(), 1);
/// assert!(!day.is_weekend());
/// # Ok::<(), helpers4::date::DateError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    /// Monday, ISO day 1.
    Monday,
    /// Tuesday, ISO day 2.
    Tuesday,
    /// Wednesday, ISO day 3.
    Wednesday,
    /// Thursday, ISO day 4.
    Thursday,
    /// Friday, ISO day 5.
    Friday,
    /// Saturday, ISO day 6.
    Saturday,
    /// Sunday, ISO day 7.
    Sunday,
}

impl Weekday {
    /// The English name, such as `"Monday"`.
    ///
    /// # Returns
    ///
    /// The capitalized name of the day.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }

    /// The ISO 8601 number of the day: Monday is `1`, Sunday is `7`.
    ///
    /// # Returns
    ///
    /// A number from `1` to `7`.
    #[must_use]
    pub fn iso_number(self) -> u8 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }

    /// Whether the day is a Saturday or a Sunday.
    ///
    /// # Returns
    ///
    /// `true` for the weekend.
    #[must_use]
    pub fn is_weekend(self) -> bool {
        matches!(self, Self::Saturday | Self::Sunday)
    }

    /// The day from its position counted from Monday (`0` is Monday, `6` is Sunday).
    pub(crate) fn from_monday_index(index: i64) -> Self {
        match index.rem_euclid(7) {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            _ => Self::Sunday,
        }
    }
}

#[cfg(test)]
#[path = "weekday.test.rs"]
mod tests;
