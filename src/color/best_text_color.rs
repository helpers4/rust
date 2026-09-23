// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Rgb, contrast_ratio};

/// Picks black or white text, whichever contrasts more with `background`.
///
/// Compares the WCAG [`contrast_ratio`](super::contrast_ratio) of both and returns the winner
/// (black on a tie), so the text stays as readable as it can be without you choosing a color per
/// background by hand.
///
/// # Arguments
///
/// - `background` - The color the text will sit on.
///
/// # Returns
///
/// Black (`#000000`) or white (`#ffffff`).
///
/// # Examples
///
/// ```
/// use helpers4::color::{best_text_color, Rgb};
///
/// assert_eq!(best_text_color(Rgb::parse("#ffeb3b")?), Rgb::new(0, 0, 0)); // on yellow
/// assert_eq!(best_text_color(Rgb::parse("#0d47a1")?), Rgb::new(255, 255, 255)); // on dark blue
/// # Ok::<(), helpers4::color::ParseColorError>(())
/// ```
#[must_use]
pub fn best_text_color(background: Rgb) -> Rgb {
    let black = Rgb::new(0, 0, 0);
    let white = Rgb::new(255, 255, 255);
    if contrast_ratio(background, black) >= contrast_ratio(background, white) {
        black
    } else {
        white
    }
}

#[cfg(test)]
#[path = "best_text_color.test.rs"]
mod tests;

#[cfg(test)]
#[path = "best_text_color.spec.rs"]
mod spec;
