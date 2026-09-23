// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Color;

/// A text style: colors and attributes, applied to a string with [`paint`](Self::paint).
///
/// Built by chaining: `Style::new().bold().fg(Color::Red)`. Painting wraps the text in the escape
/// sequence for the style and a reset (`ESC [ 0 m`); a style with nothing set returns the text
/// unchanged. This only builds the strings: whether to emit colors at all (a terminal, `NO_COLOR`)
/// is for the caller to decide.
///
/// # Examples
///
/// ```
/// use helpers4::ansi::{strip, Color, Style};
///
/// let error = Style::new().bold().fg(Color::Red);
/// assert_eq!(error.paint("failed"), "\u{1b}[1;31mfailed\u{1b}[0m");
/// assert_eq!(strip(&error.paint("failed")), "failed");
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    /// The attributes that are on, one bit each (see the `*_BIT` constants).
    attributes: u8,
}

const BOLD_BIT: u8 = 1;
const DIM_BIT: u8 = 1 << 1;
const ITALIC_BIT: u8 = 1 << 2;
const UNDERLINE_BIT: u8 = 1 << 3;
const STRIKETHROUGH_BIT: u8 = 1 << 4;

impl Style {
    /// An empty style: painting with it changes nothing.
    ///
    /// # Returns
    ///
    /// A style with no color and no attribute.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text color.
    ///
    /// # Arguments
    ///
    /// - `color` - The foreground color.
    ///
    /// # Returns
    ///
    /// The style with that text color.
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets the background color.
    ///
    /// # Arguments
    ///
    /// - `color` - The background color.
    ///
    /// # Returns
    ///
    /// The style with that background.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Makes the text bold.
    ///
    /// # Returns
    ///
    /// The style with bold on.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.attributes |= BOLD_BIT;
        self
    }

    /// Makes the text dim (faint).
    ///
    /// # Returns
    ///
    /// The style with dim on.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.attributes |= DIM_BIT;
        self
    }

    /// Makes the text italic.
    ///
    /// # Returns
    ///
    /// The style with italic on.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.attributes |= ITALIC_BIT;
        self
    }

    /// Underlines the text.
    ///
    /// # Returns
    ///
    /// The style with underline on.
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.attributes |= UNDERLINE_BIT;
        self
    }

    /// Strikes the text through.
    ///
    /// # Returns
    ///
    /// The style with strikethrough on.
    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.attributes |= STRIKETHROUGH_BIT;
        self
    }

    /// Whether the style does nothing.
    ///
    /// # Returns
    ///
    /// `true` for a style with no color and no attribute.
    #[must_use]
    pub fn is_plain(&self) -> bool {
        *self == Self::default()
    }

    /// Applies the style to `text`.
    ///
    /// # Arguments
    ///
    /// - `text` - The text to style.
    ///
    /// # Returns
    ///
    /// The text between the style's escape sequence and a reset, or the text unchanged for a plain
    /// style.
    #[must_use]
    pub fn paint(&self, text: &str) -> String {
        if self.is_plain() {
            return text.to_string();
        }
        let mut codes: Vec<String> = Vec::new();
        for (bit, code) in [
            (BOLD_BIT, "1"),
            (DIM_BIT, "2"),
            (ITALIC_BIT, "3"),
            (UNDERLINE_BIT, "4"),
            (STRIKETHROUGH_BIT, "9"),
        ] {
            if self.attributes & bit != 0 {
                codes.push(code.to_string());
            }
        }
        codes.extend(self.foreground.map(|color| color.sgr(false)));
        codes.extend(self.background.map(|color| color.sgr(true)));
        format!("\u{1b}[{}m{text}\u{1b}[0m", codes.join(";"))
    }
}

#[cfg(test)]
#[path = "style.test.rs"]
mod tests;

#[cfg(test)]
#[path = "style.spec.rs"]
mod spec;
