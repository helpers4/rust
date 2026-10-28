// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Fonts without a dependency: CSS font matching, the names design tools give to weights and
//! styles, and reading a font file's names, weight and style.
//!
//! [`pick_closest`] is CSS Fonts Level 4 matching (§5.2) among the faces of one family, with
//! variable faces that cover a weight range. [`weight_of_style`], [`is_italic`],
//! [`split_postscript`] and [`same_family`] read the names fonts and design files carry
//! (`"Semi Bold Italic"`, `"NeutrifPro-SemiBold"`). [`read_font`] reads a TrueType, OpenType or
//! WOFF file: family (typographic first), PostScript name, weight or `wght` range, style.

mod _inflate;
mod _sfnt;
#[cfg(test)]
mod _test_fixtures;
#[cfg(test)]
mod _test_support;
mod _woff;
mod font_error;
mod font_face;
mod font_info;
mod font_request;
mod font_style;
mod is_italic;
mod pick_closest;
mod read_font;
mod same_family;
mod split_postscript;
mod weight_of_style;

pub use font_error::FontError;
pub use font_face::FontFace;
pub use font_info::FontInfo;
pub use font_request::FontRequest;
pub use font_style::FontStyle;
pub use is_italic::is_italic;
pub use pick_closest::pick_closest;
pub use read_font::read_font;
pub use same_family::same_family;
pub use split_postscript::split_postscript;
pub use weight_of_style::weight_of_style;
