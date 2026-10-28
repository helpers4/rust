// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Weight names, most specific first: a longer name contains a shorter one (`extrabold` holds
/// `bold`).
const WEIGHTS: &[(&str, f64)] = &[
    ("extrablack", 950.0),
    ("ultrablack", 950.0),
    ("extrabold", 800.0),
    ("ultrabold", 800.0),
    ("extralight", 200.0),
    ("ultralight", 200.0),
    ("semibold", 600.0),
    ("demibold", 600.0),
    ("hairline", 100.0),
    ("thin", 100.0),
    ("light", 300.0),
    ("medium", 500.0),
    ("bold", 700.0),
    ("black", 900.0),
    ("heavy", 900.0),
    // The plain names come last: `Roman Bold` is a bold.
    ("regular", 400.0),
    ("normal", 400.0),
    ("book", 400.0),
    ("roman", 400.0),
];

/// The CSS weight a style name stands for, as design tools and font files write it: `Thin` 100,
/// `ExtraLight` 200, `Light` 300, `Regular` 400, `Medium` 500, `SemiBold` 600, `Bold` 700,
/// `ExtraBold` 800, `Black` 900, `ExtraBlack` 950. Case, spaces, hyphens and underscores are
/// ignored (`"Semi Bold Italic"`, `"semi-bold"`, `"SEMIBOLD"`), and so are other words such as
/// `Italic` or `Condensed`.
///
/// # Arguments
///
/// - `style` - The style name.
///
/// # Returns
///
/// The weight, or `400.0` when the name holds no weight word.
///
/// # Examples
///
/// ```
/// use helpers4::font::weight_of_style;
///
/// assert_eq!(weight_of_style("Semi Bold Italic"), 600.0);
/// assert_eq!(weight_of_style("Black"), 900.0);
/// assert_eq!(weight_of_style("Italic"), 400.0);
/// ```
#[must_use]
pub fn weight_of_style(style: &str) -> f64 {
    let squashed: String = style
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .flat_map(char::to_lowercase)
        .collect();
    WEIGHTS
        .iter()
        .find(|(name, _)| squashed.contains(name))
        .map_or(400.0, |(_, weight)| *weight)
}

#[cfg(test)]
#[path = "weight_of_style.test.rs"]
mod tests;

#[cfg(test)]
#[path = "weight_of_style.spec.rs"]
mod spec;
