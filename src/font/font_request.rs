// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::FontStyle;

/// What a text asks of a family: the properties [`pick_closest`](super::pick_closest) matches.
/// The default is CSS's initial value: weight 400, normal style, normal width.
///
/// # Examples
///
/// ```
/// use helpers4::font::{FontRequest, FontStyle};
///
/// let request = FontRequest::default();
/// assert_eq!((request.weight, request.style, request.stretch), (400.0, FontStyle::Normal, 100.0));
/// assert_eq!(FontRequest::new(600.0, FontStyle::Italic).weight, 600.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontRequest {
    /// CSS `font-weight`, 1–1000.
    pub weight: f64,
    /// CSS `font-style`.
    pub style: FontStyle,
    /// CSS `font-stretch` as a percentage.
    pub stretch: f64,
}

impl FontRequest {
    /// A request of normal width.
    ///
    /// # Arguments
    ///
    /// - `weight` - The weight, 1–1000.
    /// - `style` - The style.
    ///
    /// # Returns
    ///
    /// The request.
    #[must_use]
    pub const fn new(weight: f64, style: FontStyle) -> Self {
        Self {
            weight,
            style,
            stretch: 100.0,
        }
    }
}

impl Default for FontRequest {
    fn default() -> Self {
        Self::new(400.0, FontStyle::Normal)
    }
}

#[cfg(test)]
#[path = "font_request.test.rs"]
mod tests;
