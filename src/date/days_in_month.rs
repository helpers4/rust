// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::is_leap_year;

/// The number of days in `month` of `year`.
///
/// # Arguments
///
/// - `year` - The year, which decides whether February has 28 or 29 days.
/// - `month` - The month, `1` (January) to `12` (December).
///
/// # Returns
///
/// The number of days, or `None` when `month` is not in `1..=12`.
///
/// # Examples
///
/// ```
/// use helpers4::date::days_in_month;
///
/// assert_eq!(days_in_month(2024, 2), Some(29));
/// assert_eq!(days_in_month(2023, 2), Some(28));
/// assert_eq!(days_in_month(2023, 4), Some(30));
/// assert_eq!(days_in_month(2023, 13), None);
/// ```
#[must_use]
pub fn days_in_month(year: i32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}

#[cfg(test)]
#[path = "days_in_month.test.rs"]
mod tests;

#[cfg(test)]
#[path = "days_in_month.spec.rs"]
mod spec;
