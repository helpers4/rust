// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A terminal color for [`Style`](super::Style).
///
/// The 8 standard colors and their bright variants work everywhere; [`Color::Ansi256`] and
/// [`Color::Rgb`] need a terminal that supports 256 or true colors.
///
/// # Examples
///
/// ```
/// use helpers4::ansi::{Color, Style};
///
/// assert_eq!(Style::new().fg(Color::Green).paint("ok"), "\u{1b}[32mok\u{1b}[0m");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black.
    Black,
    /// Red.
    Red,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Blue.
    Blue,
    /// Magenta.
    Magenta,
    /// Cyan.
    Cyan,
    /// White (light grey on most terminals).
    White,
    /// Bright black (dark grey).
    BrightBlack,
    /// Bright red.
    BrightRed,
    /// Bright green.
    BrightGreen,
    /// Bright yellow.
    BrightYellow,
    /// Bright blue.
    BrightBlue,
    /// Bright magenta.
    BrightMagenta,
    /// Bright cyan.
    BrightCyan,
    /// Bright white.
    BrightWhite,
    /// A color of the 256-color palette, `0..=255`.
    Ansi256(u8),
    /// A true color.
    Rgb(u8, u8, u8),
}

impl Color {
    /// The SGR parameters that select this color: as the foreground when `background` is false,
    /// as the background otherwise (`31` or `41` for red, `38;5;n`, `38;2;r;g;b`, ...).
    pub(crate) fn sgr(self, background: bool) -> String {
        let (base, bright_base, extended) = if background {
            (40, 100, 48)
        } else {
            (30, 90, 38)
        };
        let index = |offset: u8| (base + offset).to_string();
        let bright = |offset: u8| (bright_base + offset).to_string();
        match self {
            Self::Black => index(0),
            Self::Red => index(1),
            Self::Green => index(2),
            Self::Yellow => index(3),
            Self::Blue => index(4),
            Self::Magenta => index(5),
            Self::Cyan => index(6),
            Self::White => index(7),
            Self::BrightBlack => bright(0),
            Self::BrightRed => bright(1),
            Self::BrightGreen => bright(2),
            Self::BrightYellow => bright(3),
            Self::BrightBlue => bright(4),
            Self::BrightMagenta => bright(5),
            Self::BrightCyan => bright(6),
            Self::BrightWhite => bright(7),
            Self::Ansi256(n) => format!("{extended};5;{n}"),
            Self::Rgb(r, g, b) => format!("{extended};2;{r};{g};{b}"),
        }
    }
}

#[cfg(test)]
#[path = "color.test.rs"]
mod tests;
