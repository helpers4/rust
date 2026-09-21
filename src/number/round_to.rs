// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Rounds `value` to `decimals` decimal places, half away from zero.
///
/// The result is the nearest `f64` to the rounded decimal, so it can still print with more digits
/// than requested for some values, and binary representation applies: `1.005` is stored as
/// `1.00499999999999989…`, so `round_to(1.005, 2)` is `1.0`. `NaN`, infinities, and values too
/// large to scale are returned unchanged.
///
/// # Examples
///
/// ```
/// use helpers4::number::round_to;
///
/// assert_eq!(round_to(1.23456, 2), 1.23);
/// assert_eq!(round_to(2.5, 0), 3.0);
/// assert_eq!(round_to(-2.5, 0), -3.0);
/// assert_eq!(round_to(1234.0, 0), 1234.0);
/// ```
#[must_use]
pub fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(i32::try_from(decimals).unwrap_or(i32::MAX));
    let scaled = value * factor;
    if !scaled.is_finite() {
        return value;
    }
    scaled.round() / factor
}

#[cfg(test)]
#[path = "round_to.test.rs"]
mod tests;

#[cfg(test)]
#[path = "round_to.spec.rs"]
mod spec;
