// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Median of `values`, or `None` when it is empty or contains a `NaN`.
///
/// For an even number of values it is the midpoint of the two middle ones. The input is not
/// modified.
///
/// # Examples
///
/// ```
/// use helpers4::number::median;
///
/// assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
/// assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
/// assert_eq!(median(&[]), None);
/// ```
#[must_use]
pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() || values.iter().any(|v| v.is_nan()) {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some(sorted[mid - 1].midpoint(sorted[mid]))
    } else {
        Some(sorted[mid])
    }
}

#[cfg(test)]
#[path = "median.test.rs"]
mod tests;

#[cfg(test)]
#[path = "median.spec.rs"]
mod spec;
