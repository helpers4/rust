// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Arithmetic mean of `values`, or `None` when it is empty.
///
/// `NaN` and infinities propagate as usual for `f64`.
///
/// # Examples
///
/// ```
/// use helpers4::number::mean;
///
/// assert_eq!(mean(&[1.0, 2.0, 6.0]), Some(3.0));
/// assert_eq!(mean(&[]), None);
/// ```
#[must_use]
#[allow(clippy::cast_precision_loss)] // a slice would need more than 2^53 elements to lose precision
pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

#[cfg(test)]
#[path = "mean.test.rs"]
mod tests;

#[cfg(test)]
#[path = "mean.spec.rs"]
mod spec;
