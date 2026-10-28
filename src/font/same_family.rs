// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

fn squash(name: &str) -> impl Iterator<Item = char> + '_ {
    name.chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
}

/// Whether two family names are the same family: compared without spaces or case, since design
/// files and font files write `"SF Pro"`, `"SFPro"` and `"sf pro"` for one family.
///
/// # Arguments
///
/// - `a` - A family name.
/// - `b` - Another one.
///
/// # Returns
///
/// `true` if the names are equal once spaces are removed and case is ignored.
///
/// # Examples
///
/// ```
/// use helpers4::font::same_family;
///
/// assert!(same_family("SF Pro", "sfpro"));
/// assert!(!same_family("SF Pro", "SF Mono"));
/// ```
#[must_use]
pub fn same_family(a: &str, b: &str) -> bool {
    squash(a).eq(squash(b))
}

#[cfg(test)]
#[path = "same_family.test.rs"]
mod tests;

#[cfg(test)]
#[path = "same_family.spec.rs"]
mod spec;
