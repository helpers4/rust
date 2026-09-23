// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgb;

/// The WCAG contrast ratio between two colors, from `1.0` (identical) to `21.0` (black on white).
///
/// It is `(L1 + 0.05) / (L2 + 0.05)` where `L1` is the relative luminance of the lighter color
/// and `L2` of the darker one, so the order of the arguments does not matter. WCAG 2.x asks for
/// at least `4.5` for normal text and `3.0` for large text (level AA), `7.0` and `4.5` for AAA.
///
/// # Arguments
///
/// - `a` - One color, for instance the text.
/// - `b` - The other color, for instance its background.
///
/// # Returns
///
/// The ratio, `1.0..=21.0`.
///
/// # Examples
///
/// ```
/// use helpers4::color::{contrast_ratio, Rgb};
///
/// let ratio = contrast_ratio(Rgb::new(0, 0, 0), Rgb::new(255, 255, 255));
/// assert!((ratio - 21.0).abs() < 1e-9);
/// assert!(contrast_ratio(Rgb::parse("#767676")?, Rgb::new(255, 255, 255)) >= 4.5);
/// # Ok::<(), helpers4::color::ParseColorError>(())
/// ```
#[must_use]
pub fn contrast_ratio(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    let (lighter, darker) = if la >= lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

#[cfg(test)]
#[path = "contrast_ratio.test.rs"]
mod tests;

#[cfg(test)]
#[path = "contrast_ratio.spec.rs"]
mod spec;
