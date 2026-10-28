// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{FontFace, FontStyle};

/// What a font file says about itself, as [`read_font`](super::read_font) reads it.
///
/// # Examples
///
/// ```
/// use helpers4::font::{FontFace, FontInfo, FontStyle};
///
/// let info = FontInfo {
///     family: "Inter".to_string(),
///     style: "Bold Italic".to_string(),
///     postscript_name: Some("Inter-BoldItalic".to_string()),
///     weight: 700.0,
///     weight_range: None,
///     italic: true,
/// };
/// assert_eq!(info.to_face(), FontFace::new(700.0, FontStyle::Italic));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FontInfo {
    /// The family name: the typographic family when the font has one, else the plain family.
    pub family: String,
    /// The style name (`Semi Bold Italic`): typographic first, empty when the font has none.
    pub style: String,
    /// The PostScript name (`Inter-BoldItalic`).
    pub postscript_name: Option<String>,
    /// The weight of a static font: the OS/2 weight class, else read from the style name. For a
    /// variable font, the default instance.
    pub weight: f64,
    /// The range of the `wght` axis of a variable font.
    pub weight_range: Option<(f64, f64)>,
    /// Whether the font is italic or oblique.
    pub italic: bool,
}

impl FontInfo {
    /// The face to give to [`pick_closest`](super::pick_closest): a variable font covers its
    /// `wght` range, a static one its weight.
    ///
    /// # Returns
    ///
    /// The face, italic when the font is.
    #[must_use]
    pub fn to_face(&self) -> FontFace {
        let style = if self.italic {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        };
        let (min, max) = self.weight_range.unwrap_or((self.weight, self.weight));
        FontFace::variable(min, max, style)
    }
}

#[cfg(test)]
#[path = "font_info.test.rs"]
mod tests;
