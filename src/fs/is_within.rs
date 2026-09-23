// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_parts::parts;
use std::path::{Component, Path};

/// Whether `path`, taken relative to `base`, stays inside `base`: a guard against path traversal.
///
/// A relative `path` is joined to `base` and both are cleaned lexically (see
/// [`normalize`](super::normalize)), so `"a/../../b"` escapes and `"a/../b"` does not; an
/// absolute `path` must itself lie under `base`. `base` counts as inside itself. Nothing touches
/// the file system, so this does **not** see symbolic links: if the directory can contain links
/// an attacker controls, resolve the path with [`std::fs::canonicalize`] and compare that instead.
///
/// # Arguments
///
/// - `base` - The directory that must contain the result.
/// - `path` - The path to check, relative to `base` or absolute.
///
/// # Returns
///
/// `true` when the cleaned path is `base` or lies under it.
///
/// # Examples
///
/// ```
/// use helpers4::fs::is_within;
/// use std::path::Path;
///
/// let uploads = Path::new("/srv/uploads");
/// assert!(is_within(uploads, Path::new("avatars/me.png")));
/// assert!(!is_within(uploads, Path::new("../secrets.txt")));
/// assert!(!is_within(uploads, Path::new("/etc/passwd")));
/// ```
#[must_use]
pub fn is_within(base: &Path, path: &Path) -> bool {
    let base_parts = parts(base);
    let joined = base.join(path);
    let full = parts(&joined);
    full.starts_with(&base_parts)
        && full[base_parts.len()..]
            .iter()
            .all(|component| matches!(component, Component::Normal(_)))
}

#[cfg(test)]
#[path = "is_within.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_within.spec.rs"]
mod spec;
