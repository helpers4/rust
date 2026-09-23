// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_named::NAMED;
use super::{Hsl, ParseColorError};
use std::fmt;
use std::str::FromStr;

/// An sRGB color with 8 bits per channel.
///
/// Built with [`Rgb::new`] or parsed from text with [`Rgb::parse`] (`#rgb`, `#rrggbb`,
/// `rgb(r, g, b)` or one of the 17 basic CSS color names). `Display` writes the `#rrggbb` form.
///
/// # Examples
///
/// ```
/// use helpers4::color::Rgb;
///
/// let orange = Rgb::parse("#ff8800")?;
/// assert_eq!((orange.r(), orange.g(), orange.b()), (255, 136, 0));
/// assert_eq!(orange.to_string(), "#ff8800");
/// assert_eq!(Rgb::parse("rgb(0, 128, 255)")?, Rgb::new(0, 128, 255));
/// assert_eq!(Rgb::parse("Red")?.to_hex(), "#ff0000");
/// # Ok::<(), helpers4::color::ParseColorError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// Builds a color from its three channels.
    ///
    /// # Arguments
    ///
    /// - `r` - Red, `0..=255`.
    /// - `g` - Green, `0..=255`.
    /// - `b` - Blue, `0..=255`.
    ///
    /// # Returns
    ///
    /// The color.
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parses a color written as `#rgb`, `#rrggbb`, `rgb(r, g, b)` or a basic CSS color name.
    ///
    /// Surrounding whitespace is ignored, hex digits and names are case-insensitive. `#rgb`
    /// repeats each digit (`#f80` is `#ff8800`). In `rgb(...)` the three whole numbers (`0..=255`)
    /// may be separated by commas or spaces. The names are the 17 basic CSS keywords: `black`,
    /// `silver`, `gray`, `white`, `maroon`, `red`, `purple`, `fuchsia`, `green`, `lime`, `olive`,
    /// `yellow`, `navy`, `blue`, `teal`, `aqua` and `orange`.
    ///
    /// # Arguments
    ///
    /// - `text` - The color to parse.
    ///
    /// # Errors
    ///
    /// A [`ParseColorError`] for an empty text, a `#` color with a bad length or digit, a
    /// malformed or out-of-range `rgb(...)`, or an unknown name.
    pub fn parse(text: &str) -> Result<Self, ParseColorError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(ParseColorError::Empty);
        }
        if let Some(hex) = text.strip_prefix('#') {
            return Self::parse_hex(hex);
        }
        let lower = text.to_ascii_lowercase();
        if let Some(inner) = lower
            .strip_prefix("rgb(")
            .and_then(|rest| rest.strip_suffix(')'))
        {
            return Self::parse_function(inner);
        }
        NAMED
            .iter()
            .find(|(name, _)| *name == lower)
            .map(|(_, [r, g, b])| Self::new(*r, *g, *b))
            .ok_or(ParseColorError::UnknownName)
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseColorError> {
        if let Some(offset) = hex.bytes().position(|b| !b.is_ascii_hexdigit()) {
            return Err(ParseColorError::InvalidDigit { index: offset + 1 });
        }
        let digit = |i: usize| digit_value(hex.as_bytes()[i]);
        match hex.len() {
            3 => Ok(Self::new(digit(0) * 17, digit(1) * 17, digit(2) * 17)),
            6 => Ok(Self::new(
                digit(0) * 16 + digit(1),
                digit(2) * 16 + digit(3),
                digit(4) * 16 + digit(5),
            )),
            length => Err(ParseColorError::InvalidLength { length }),
        }
    }

    fn parse_function(inner: &str) -> Result<Self, ParseColorError> {
        let numbers: Vec<&str> = inner
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .collect();
        let [r, g, b] = numbers[..] else {
            return Err(ParseColorError::InvalidFunction);
        };
        let channel = |text: &str| -> Result<u8, ParseColorError> {
            let value: u16 = text.parse().map_err(|_| ParseColorError::InvalidFunction)?;
            u8::try_from(value).map_err(|_| ParseColorError::OutOfRange)
        };
        Ok(Self::new(channel(r)?, channel(g)?, channel(b)?))
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

    /// The `#rrggbb` form, in lower case.
    ///
    /// # Returns
    ///
    /// For instance `"#ff8800"`.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// The same color as hue, saturation and lightness.
    ///
    /// # Returns
    ///
    /// The [`Hsl`] value. Converting it back with [`Hsl::to_rgb`] gives this color again.
    #[must_use]
    pub fn to_hsl(self) -> Hsl {
        let (max, min) = (
            self.r.max(self.g).max(self.b),
            self.r.min(self.g).min(self.b),
        );
        let (r, g, b) = (
            f64::from(self.r) / 255.0,
            f64::from(self.g) / 255.0,
            f64::from(self.b) / 255.0,
        );
        let lightness = (f64::from(max) + f64::from(min)) / 510.0;
        let delta = f64::from(max - min) / 255.0;
        if delta <= 0.0 {
            return Hsl::new(0.0, 0.0, lightness);
        }
        let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());
        let sector = if max == self.r {
            ((g - b) / delta).rem_euclid(6.0)
        } else if max == self.g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        Hsl::new(sector * 60.0, saturation, lightness)
    }

    /// The WCAG relative luminance: `0.0` for black, `1.0` for white.
    ///
    /// The channels are linearized from sRGB and weighted by how bright the eye finds each
    /// (0.2126 red, 0.7152 green, 0.0722 blue).
    ///
    /// # Returns
    ///
    /// A value from `0.0` to `1.0`.
    #[must_use]
    pub fn luminance(self) -> f64 {
        let linear = |channel: u8| {
            let c = f64::from(channel) / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(self.r) + 0.7152 * linear(self.g) + 0.0722 * linear(self.b)
    }

    /// Moves the color towards white.
    ///
    /// # Arguments
    ///
    /// - `amount` - How far to go, from `0.0` (unchanged) to `1.0` (white); out-of-range values
    ///   are clamped.
    ///
    /// # Returns
    ///
    /// The lighter color.
    #[must_use]
    pub fn lighten(self, amount: f64) -> Self {
        super::mix(self, Self::new(255, 255, 255), amount)
    }

    /// Moves the color towards black.
    ///
    /// # Arguments
    ///
    /// - `amount` - How far to go, from `0.0` (unchanged) to `1.0` (black); out-of-range values
    ///   are clamped.
    ///
    /// # Returns
    ///
    /// The darker color.
    #[must_use]
    pub fn darken(self, amount: f64) -> Self {
        super::mix(self, Self::new(0, 0, 0), amount)
    }
}

/// The value of a hexadecimal digit already known to be one.
fn digit_value(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => byte - b'A' + 10,
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl FromStr for Rgb {
    type Err = ParseColorError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "rgb.test.rs"]
mod tests;

#[cfg(test)]
#[path = "rgb.spec.rs"]
mod spec;
