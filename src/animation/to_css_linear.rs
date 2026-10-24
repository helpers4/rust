// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::internal::css_number::trimmed;

/// Samples an easing function into a CSS `linear()` easing, so that any curve (a
/// [`Spring`](super::Spring), a custom bounce) can be played by CSS transitions and the Web
/// Animations API, which do not take code.
///
/// Values are rounded to 4 decimals. More samples follow the curve more closely, in a longer
/// string; 32 is smooth for most springs.
///
/// # Arguments
///
/// - `easing` - The easing function over `0.0..=1.0`.
/// - `samples` - Number of points, evenly spaced in time, at least `2`.
///
/// # Returns
///
/// A `linear(...)` string, or `None` when `samples` is less than `2`.
///
/// # Examples
///
/// ```
/// use helpers4::animation::{to_css_linear, Spring};
///
/// assert_eq!(
///     to_css_linear(&|t| t * t, 5).as_deref(),
///     Some("linear(0, 0.0625, 0.25, 0.5625, 1)")
/// );
///
/// let spring = Spring::default();
/// let css = to_css_linear(&|t| spring.easing(t), 32);
/// assert!(css.is_some_and(|css| css.starts_with("linear(0, ")));
/// ```
#[must_use]
pub fn to_css_linear(easing: &dyn Fn(f64) -> f64, samples: u32) -> Option<String> {
    if samples < 2 {
        return None;
    }
    let last = f64::from(samples - 1);
    let values: Vec<String> = (0..samples)
        .map(|index| trimmed(easing(f64::from(index) / last), 4))
        .collect();
    Some(format!("linear({})", values.join(", ")))
}

#[cfg(test)]
#[path = "to_css_linear.test.rs"]
mod tests;

#[cfg(test)]
#[path = "to_css_linear.spec.rs"]
mod spec;
