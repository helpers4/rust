// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// What percent `part` is of `total`, or `None` when `total` is zero.
///
/// The result is not clamped: a `part` larger than `total` gives more than 100.
///
/// # Arguments
///
/// - `part` - The quantity to express as a percentage.
/// - `total` - The whole that `part` is a share of.
///
/// # Examples
///
/// ```
/// use helpers4::number::percentage;
///
/// assert_eq!(percentage(25.0, 200.0), Some(12.5));
/// assert_eq!(percentage(3.0, 2.0), Some(150.0));
/// assert_eq!(percentage(1.0, 0.0), None);
/// ```
#[must_use]
pub fn percentage(part: f64, total: f64) -> Option<f64> {
    if total == 0.0 {
        return None;
    }
    Some(part / total * 100.0)
}

#[cfg(test)]
#[path = "percentage.test.rs"]
mod tests;
