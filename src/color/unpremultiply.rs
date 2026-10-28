// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgba;

/// Divides the color channels by the alpha, the inverse of [`premultiply`](super::premultiply).
/// A fully transparent color (alpha `0`) holds no color, so it gives transparent black. Channels
/// are clamped to 255 and rounded to 8 bits.
///
/// # Arguments
///
/// - `color` - The premultiplied color.
///
/// # Returns
///
/// The straight color, with the same alpha.
///
/// # Examples
///
/// ```
/// use helpers4::color::{unpremultiply, Rgba};
///
/// assert_eq!(unpremultiply(Rgba::new(100, 50, 25, 128)), Rgba::new(199, 100, 50, 128));
/// assert_eq!(unpremultiply(Rgba::new(9, 9, 9, 0)), Rgba::new(0, 0, 0, 0));
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to 0..=255 first
pub fn unpremultiply(color: Rgba) -> Rgba {
    if color.a() == 0 {
        return Rgba::new(0, 0, 0, 0);
    }
    let scale = |channel: u8| {
        (f64::from(channel) / color.opacity())
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Rgba::new(
        scale(color.r()),
        scale(color.g()),
        scale(color.b()),
        color.a(),
    )
}

#[cfg(test)]
#[path = "unpremultiply.test.rs"]
mod tests;

#[cfg(test)]
#[path = "unpremultiply.spec.rs"]
mod spec;
