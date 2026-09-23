// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Day-count arithmetic for the proleptic Gregorian calendar (Howard Hinnant's `days_from_civil`
//! and `civil_from_days`). Days are counted from 1970-01-01, which is day 0. Not re-exported.

/// Days since 1970-01-01 of the given civil date. `month` is 1..=12 and `day` 1..=31.
pub(crate) fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let month = i64::from(month);
    let year = i64::from(year) - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The civil date `(year, month, day)` of a day count since 1970-01-01.
///
/// The count must lie within the years 0..=9999 (-719528..=2932896), which keeps every result in
/// the range of the narrow types it is converted to.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn civil_from_days(days: i64) -> (i32, u8, u8) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year as i32, month as u8, day as u8)
}

#[cfg(test)]
#[path = "_civil.test.rs"]
mod tests;
