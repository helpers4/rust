// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_table::{DEPRECATED, TABLE};
use super::{Category, LicenseInfo};

/// What this module knows about the license with SPDX identifier `id`.
///
/// The match ignores case (SPDX identifiers are case-insensitive) and accepts the deprecated
/// identifiers that are still common (`GPL-3.0`, `LGPL-2.1+`, `AGPL-3.0`, ...), answering with the
/// current one (see [`normalize`](super::normalize)). About forty common licenses are known; for
/// any other identifier the answer is `None`, which means "not in this list", not "invalid".
///
/// # Arguments
///
/// - `id` - The SPDX identifier, such as `"MIT"` or `"Apache-2.0"`.
///
/// # Returns
///
/// The [`LicenseInfo`], or `None` for an identifier that is not known.
///
/// # Examples
///
/// ```
/// use helpers4::license::{lookup, Category};
///
/// assert_eq!(lookup("MPL-2.0").map(|i| i.category()), Some(Category::WeakCopyleft));
/// assert_eq!(lookup("gpl-3.0").map(|i| i.id()), Some("GPL-3.0-only"));
/// assert_eq!(lookup("Not-A-License"), None);
/// ```
#[must_use]
pub fn lookup(id: &str) -> Option<LicenseInfo> {
    let id = DEPRECATED
        .iter()
        .find(|(old, _)| old.eq_ignore_ascii_case(id))
        .map_or(id, |(_, current)| current);
    TABLE
        .iter()
        .find(|(known, _, _): &&(&str, &str, Category)| known.eq_ignore_ascii_case(id))
        .map(|(known, name, category)| LicenseInfo::new(known, name, *category))
}

#[cfg(test)]
#[path = "lookup.test.rs"]
mod tests;
