// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Whether a style name is italic or oblique, as design tools and font files write it
/// (`"Italic"`, `"Semi Bold Italic"`, `"Light Oblique"`). Case is ignored.
///
/// # Arguments
///
/// - `style` - The style name.
///
/// # Returns
///
/// `true` if the name holds `italic` or `oblique`.
///
/// # Examples
///
/// ```
/// use helpers4::font::is_italic;
///
/// assert!(is_italic("Semi Bold Italic"));
/// assert!(is_italic("OBLIQUE"));
/// assert!(!is_italic("Bold"));
/// ```
#[must_use]
pub fn is_italic(style: &str) -> bool {
    let lower = style.to_lowercase();
    lower.contains("italic") || lower.contains("oblique")
}

#[cfg(test)]
#[path = "is_italic.test.rs"]
mod tests;
