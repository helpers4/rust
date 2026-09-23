// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_table::DEPRECATED;

/// The current SPDX identifier for a deprecated one.
///
/// SPDX split the GNU licenses into `-only` and `-or-later` and retired the bare forms and the
/// `+` suffix, but `GPL-3.0` and `LGPL-2.1+` are still what many manifests contain. The match
/// ignores case. Identifiers that are not deprecated give `None`.
///
/// # Arguments
///
/// - `id` - The identifier to update.
///
/// # Returns
///
/// The replacement, such as `"GPL-3.0-only"` for `"GPL-3.0"`, or `None` when `id` is not one of
/// the ten deprecated GNU identifiers (`GPL`, `LGPL` and `AGPL`, with and without `+`).
///
/// # Examples
///
/// ```
/// use helpers4::license::normalize;
///
/// assert_eq!(normalize("GPL-3.0"), Some("GPL-3.0-only"));
/// assert_eq!(normalize("GPL-3.0+"), Some("GPL-3.0-or-later"));
/// assert_eq!(normalize("MIT"), None);
/// ```
#[must_use]
pub fn normalize(id: &str) -> Option<&'static str> {
    DEPRECATED
        .iter()
        .find(|(old, _)| old.eq_ignore_ascii_case(id))
        .map(|(_, current)| *current)
}

#[cfg(test)]
#[path = "normalize.test.rs"]
mod tests;
