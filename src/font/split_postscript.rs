// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Splits a PostScript font name into family and style at its first hyphen:
/// `NeutrifPro-SemiBold` is `NeutrifPro` and `SemiBold`.
///
/// # Arguments
///
/// - `name` - The PostScript name.
///
/// # Returns
///
/// The family and the style, borrowed from `name`. The style is empty when there is no hyphen.
///
/// # Examples
///
/// ```
/// use helpers4::font::split_postscript;
///
/// assert_eq!(split_postscript("NeutrifPro-SemiBold"), ("NeutrifPro", "SemiBold"));
/// assert_eq!(split_postscript("Helvetica"), ("Helvetica", ""));
/// ```
#[must_use]
pub fn split_postscript(name: &str) -> (&str, &str) {
    name.split_once('-').unwrap_or((name, ""))
}

#[cfg(test)]
#[path = "split_postscript.test.rs"]
mod tests;
