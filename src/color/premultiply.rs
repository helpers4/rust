// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgba;

/// Multiplies the color channels by the alpha, the form a renderer composites with (a
/// translucent color then adds to what is underneath without a further multiplication).
/// Channels are rounded to 8 bits, so [`unpremultiply`](super::unpremultiply) gives back the
/// original only approximately for translucent colors, and not at all for alpha `0`.
///
/// # Arguments
///
/// - `color` - The straight color.
///
/// # Returns
///
/// The premultiplied color, with the same alpha.
///
/// # Examples
///
/// ```
/// use helpers4::color::{premultiply, Rgba};
///
/// assert_eq!(premultiply(Rgba::new(200, 100, 50, 128)), Rgba::new(100, 50, 25, 128));
/// assert_eq!(premultiply(Rgba::new(200, 100, 50, 255)), Rgba::new(200, 100, 50, 255));
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // at most 255 * 255 / 255
pub fn premultiply(color: Rgba) -> Rgba {
    let scale = |channel: u8| (f64::from(channel) * color.opacity()).round() as u8;
    Rgba::new(
        scale(color.r()),
        scale(color.g()),
        scale(color.b()),
        color.a(),
    )
}

#[cfg(test)]
#[path = "premultiply.test.rs"]
mod tests;

#[cfg(test)]
#[path = "premultiply.spec.rs"]
mod spec;
