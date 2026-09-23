// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Whether `year` is a leap year in the Gregorian calendar.
///
/// A year is leap when it is divisible by 4, except centuries, which must be divisible by 400:
/// 2000 and 2024 are leap, 1900 and 2023 are not.
///
/// # Arguments
///
/// - `year` - The year to check.
///
/// # Returns
///
/// `true` for a leap year.
///
/// # Examples
///
/// ```
/// use helpers4::date::is_leap_year;
///
/// assert!(is_leap_year(2024));
/// assert!(is_leap_year(2000));
/// assert!(!is_leap_year(1900));
/// assert!(!is_leap_year(2023));
/// ```
#[must_use]
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
#[path = "is_leap_year.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_leap_year.spec.rs"]
mod spec;
