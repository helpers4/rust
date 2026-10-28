// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The `font-style` of a face or a request. Oblique angles are not compared.
///
/// # Examples
///
/// ```
/// use helpers4::font::FontStyle;
///
/// assert_eq!(FontStyle::default(), FontStyle::Normal);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum FontStyle {
    /// Upright. The default.
    #[default]
    Normal,
    /// A true italic.
    Italic,
    /// A slanted upright.
    Oblique,
}
