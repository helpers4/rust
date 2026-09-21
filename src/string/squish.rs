// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Trims `s` and collapses every run of whitespace into a single space.
///
/// Tabs and line breaks count as whitespace, so a multi-line text becomes one line.
///
/// # Examples
///
/// ```
/// use helpers4::string::squish;
///
/// assert_eq!(squish("  hello   \n\t world  "), "hello world");
/// assert_eq!(squish("   "), "");
/// ```
#[must_use]
pub fn squish(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
#[path = "squish.test.rs"]
mod tests;

#[cfg(test)]
#[path = "squish.spec.rs"]
mod spec;
