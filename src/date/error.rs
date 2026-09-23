// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a date could not be built or parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateError {
    /// The year is outside the supported range `0..=9999`.
    InvalidYear {
        /// The rejected year.
        year: i32,
    },
    /// The month is not in `1..=12`.
    InvalidMonth {
        /// The rejected month.
        month: u8,
    },
    /// The day is not in `1..=max` for that month.
    InvalidDay {
        /// The rejected day.
        day: u8,
        /// The number of days in that month.
        max: u8,
    },
    /// The text is not exactly `YYYY-MM-DD` with ASCII digits.
    InvalidFormat,
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidYear { year } => write!(f, "year {year} is outside 0..=9999"),
            Self::InvalidMonth { month } => write!(f, "month {month} is not in 1..=12"),
            Self::InvalidDay { day, max } => write!(f, "day {day} is not in 1..={max}"),
            Self::InvalidFormat => write!(f, "expected a date written YYYY-MM-DD"),
        }
    }
}

impl std::error::Error for DateError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
