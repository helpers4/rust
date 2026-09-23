// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Calendar dates without time zones: validation, ISO 8601 text, weekdays and day arithmetic.
//!
//! A [`Date`] is a plain year-month-day in the proleptic Gregorian calendar, limited to the years
//! `0..=9999`. Nothing here reads the clock or knows about time zones: convert a unix time to a day
//! count and pass it in.

mod _civil;
mod calendar_date;
mod days_in_month;
mod error;
mod is_leap_year;
mod weekday;

pub use calendar_date::Date;
pub use days_in_month::days_in_month;
pub use error::DateError;
pub use is_leap_year::is_leap_year;
pub use weekday::Weekday;
