// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Linear interpolation between `from` and `to`: `from` at `t = 0`, `to` at `t = 1`.
///
/// `t` is not clamped, so values outside `0..=1` extrapolate. Written as
/// `from * (1 - t) + to * t`, it is exact at both ends.
///
/// # Examples
///
/// ```
/// use helpers4::number::lerp;
///
/// assert_eq!(lerp(10.0, 20.0, 0.5), 15.0);
/// assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
/// assert_eq!(lerp(0.0, 10.0, 1.5), 15.0);
/// ```
#[must_use]
pub fn lerp(from: f64, to: f64, t: f64) -> f64 {
    from * (1.0 - t) + to * t
}

#[cfg(test)]
#[path = "lerp.test.rs"]
mod tests;

#[cfg(test)]
#[path = "lerp.spec.rs"]
mod spec;
