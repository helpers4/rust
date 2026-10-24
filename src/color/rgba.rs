// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{ParseColorError, Rgb};
use crate::internal::css_number::trimmed;
use std::fmt;
use std::str::FromStr;

/// An sRGB color with 8 bits per channel and an alpha (opacity) channel.
///
/// [`Rgb`] is always opaque; `Rgba` adds the alpha of `#rrggbbaa` and `rgb(r g b / a)`. The alpha
/// is a `u8` like the other channels: `255` is opaque, `0` fully transparent. `Display` writes
/// the `#rrggbb` form when opaque and `#rrggbbaa` otherwise.
///
/// # Examples
///
/// ```
/// use helpers4::color::{Rgb, Rgba};
///
/// let glass = Rgba::parse("#ff880080")?;
/// assert_eq!((glass.r(), glass.g(), glass.b(), glass.a()), (255, 136, 0, 128));
/// assert_eq!(glass.to_css(), "rgb(255 136 0 / 0.502)");
/// assert_eq!(glass.to_rgb(), Rgb::new(255, 136, 0)); // drops the alpha
/// assert_eq!(Rgba::parse("rgb(255 136 0 / 50%)")?.a(), 128);
/// # Ok::<(), helpers4::color::ParseColorError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    /// Builds a color from its four channels.
    ///
    /// # Arguments
    ///
    /// - `r` - Red, `0..=255`.
    /// - `g` - Green, `0..=255`.
    /// - `b` - Blue, `0..=255`.
    /// - `a` - Alpha, `0` (transparent) to `255` (opaque).
    ///
    /// # Returns
    ///
    /// The color.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::Rgba;
    ///
    /// assert_eq!(Rgba::new(255, 0, 0, 255).to_hex(), "#ff0000");
    /// ```
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Adds an alpha to an opaque color.
    ///
    /// # Arguments
    ///
    /// - `rgb` - The color.
    /// - `a` - Alpha, `0` (transparent) to `255` (opaque).
    ///
    /// # Returns
    ///
    /// The color with that alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::{Rgb, Rgba};
    ///
    /// assert_eq!(Rgba::from_rgb(Rgb::new(1, 2, 3), 4), Rgba::new(1, 2, 3, 4));
    /// ```
    #[must_use]
    pub fn from_rgb(rgb: Rgb, a: u8) -> Self {
        Self::new(rgb.r(), rgb.g(), rgb.b(), a)
    }

    /// Parses a color written as `#rgba`, `#rrggbbaa`, `rgb(r g b / a)`, `rgb(r, g, b, a)`,
    /// `rgba(...)`, or any form [`Rgb::parse`] reads (which is opaque).
    ///
    /// The alpha of a function is a number (`0.5`) or a percentage (`50%`); like in CSS, a value
    /// outside `0..=1` is clamped. Surrounding whitespace is ignored, and so is case.
    ///
    /// # Arguments
    ///
    /// - `text` - The color to parse.
    ///
    /// # Returns
    ///
    /// The color.
    ///
    /// # Errors
    ///
    /// A [`ParseColorError`] for an empty text, a `#` color with a bad length or digit, a
    /// malformed or out-of-range function, or an unknown name.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::Rgba;
    ///
    /// assert_eq!(Rgba::parse("#f808")?, Rgba::new(255, 136, 0, 136));
    /// assert_eq!(Rgba::parse("rgba(0, 0, 0, 0.5)")?, Rgba::new(0, 0, 0, 128));
    /// assert_eq!(Rgba::parse("red")?, Rgba::new(255, 0, 0, 255));
    /// # Ok::<(), helpers4::color::ParseColorError>(())
    /// ```
    pub fn parse(text: &str) -> Result<Self, ParseColorError> {
        let trimmed_text = text.trim();
        if let Some(hex) = trimmed_text.strip_prefix('#') {
            if matches!(hex.len(), 4 | 8) {
                return Self::parse_hex(hex);
            }
        }
        let lower = trimmed_text.to_ascii_lowercase();
        let inner = lower
            .strip_prefix("rgba(")
            .or_else(|| lower.strip_prefix("rgb("))
            .and_then(|rest| rest.strip_suffix(')'));
        match inner {
            Some(inner) => Self::parse_function(inner),
            None => Rgb::parse(trimmed_text).map(|rgb| Self::from_rgb(rgb, 255)),
        }
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseColorError> {
        if let Some(offset) = hex.bytes().position(|b| !b.is_ascii_hexdigit()) {
            return Err(ParseColorError::InvalidDigit { index: offset + 1 });
        }
        let digit = |i: usize| digit_value(hex.as_bytes()[i]);
        if hex.len() == 4 {
            Ok(Self::new(
                digit(0) * 17,
                digit(1) * 17,
                digit(2) * 17,
                digit(3) * 17,
            ))
        } else {
            Ok(Self::new(
                digit(0) * 16 + digit(1),
                digit(2) * 16 + digit(3),
                digit(4) * 16 + digit(5),
                digit(6) * 16 + digit(7),
            ))
        }
    }

    fn parse_function(inner: &str) -> Result<Self, ParseColorError> {
        // `r g b / a`, `r, g, b, a`, or just the three channels (opaque).
        let (colors, alpha) = if let Some((colors, alpha)) = inner.split_once('/') {
            (split_numbers(colors), Some(alpha.trim()))
        } else {
            let mut all = split_numbers(inner);
            let alpha = if all.len() == 4 { all.pop() } else { None };
            (all, alpha)
        };
        let [r, g, b] = colors[..] else {
            return Err(ParseColorError::InvalidFunction);
        };
        let channel = |text: &str| -> Result<u8, ParseColorError> {
            let value: u16 = text.parse().map_err(|_| ParseColorError::InvalidFunction)?;
            u8::try_from(value).map_err(|_| ParseColorError::OutOfRange)
        };
        let alpha = match alpha {
            Some(text) => parse_alpha(text)?,
            None => 255,
        };
        Ok(Self::new(channel(r)?, channel(g)?, channel(b)?, alpha))
    }

    /// The red channel.
    ///
    /// # Returns
    ///
    /// A value from `0` to `255`.
    #[must_use]
    pub fn r(self) -> u8 {
        self.r
    }

    /// The green channel.
    ///
    /// # Returns
    ///
    /// A value from `0` to `255`.
    #[must_use]
    pub fn g(self) -> u8 {
        self.g
    }

    /// The blue channel.
    ///
    /// # Returns
    ///
    /// A value from `0` to `255`.
    #[must_use]
    pub fn b(self) -> u8 {
        self.b
    }

    /// The alpha channel.
    ///
    /// # Returns
    ///
    /// A value from `0` (transparent) to `255` (opaque).
    #[must_use]
    pub fn a(self) -> u8 {
        self.a
    }

    /// The alpha as an opacity.
    ///
    /// # Returns
    ///
    /// A value from `0.0` (transparent) to `1.0` (opaque).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::Rgba;
    ///
    /// assert_eq!(Rgba::new(0, 0, 0, 255).opacity(), 1.0);
    /// assert_eq!(Rgba::new(0, 0, 0, 0).opacity(), 0.0);
    /// ```
    #[must_use]
    pub fn opacity(self) -> f64 {
        f64::from(self.a) / 255.0
    }

    /// The same color without its alpha.
    ///
    /// # Returns
    ///
    /// The opaque [`Rgb`].
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::{Rgb, Rgba};
    ///
    /// assert_eq!(Rgba::new(1, 2, 3, 4).to_rgb(), Rgb::new(1, 2, 3));
    /// ```
    #[must_use]
    pub fn to_rgb(self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
    }

    /// The `#rrggbb` form when opaque, `#rrggbbaa` otherwise, in lower case.
    ///
    /// # Returns
    ///
    /// For instance `"#ff8800"` or `"#ff880080"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::Rgba;
    ///
    /// assert_eq!(Rgba::new(255, 136, 0, 128).to_hex(), "#ff880080");
    /// ```
    #[must_use]
    pub fn to_hex(self) -> String {
        if self.a == 255 {
            self.to_rgb().to_hex()
        } else {
            format!("{}{:02x}", self.to_rgb().to_hex(), self.a)
        }
    }

    /// The CSS form: `rgb(r g b)` when opaque, `rgb(r g b / alpha)` otherwise, with the alpha
    /// as an opacity rounded to 3 decimals.
    ///
    /// # Returns
    ///
    /// For instance `"rgb(255 136 0 / 0.502)"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::color::Rgba;
    ///
    /// assert_eq!(Rgba::new(255, 136, 0, 255).to_css(), "rgb(255 136 0)");
    /// assert_eq!(Rgba::new(255, 136, 0, 0).to_css(), "rgb(255 136 0 / 0)");
    /// ```
    #[must_use]
    pub fn to_css(self) -> String {
        if self.a == 255 {
            self.to_rgb().to_css()
        } else {
            format!(
                "rgb({} {} {} / {})",
                self.r,
                self.g,
                self.b,
                trimmed(self.opacity(), 3)
            )
        }
    }
}

/// The numbers of a color function, separated by commas or whitespace.
fn split_numbers(text: &str) -> Vec<&str> {
    text.split(|c: char| c == ',' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .collect()
}

/// An alpha written as a number (`0.5`) or a percentage (`50%`), clamped to `0..=1` like in CSS.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to 0..=255 first
fn parse_alpha(text: &str) -> Result<u8, ParseColorError> {
    let (number, scale) = match text.strip_suffix('%') {
        Some(percent) => (percent, 100.0),
        None => (text, 1.0),
    };
    let value: f64 = number
        .parse()
        .map_err(|_| ParseColorError::InvalidFunction)?;
    if !value.is_finite() {
        return Err(ParseColorError::InvalidFunction);
    }
    Ok(((value / scale).clamp(0.0, 1.0) * 255.0).round() as u8)
}

/// The value of a hexadecimal digit already known to be one.
fn digit_value(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => byte - b'A' + 10,
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Self::from_rgb(rgb, 255)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl FromStr for Rgba {
    type Err = ParseColorError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "rgba.test.rs"]
mod tests;

#[cfg(test)]
#[path = "rgba.spec.rs"]
mod spec;
