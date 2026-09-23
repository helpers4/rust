// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_parts::parts;
use std::path::{Path, PathBuf};

/// Cleans a path lexically: drops `.` components and folds `..` into the component before it.
///
/// Nothing touches the file system, so this works for paths that do not exist, but symbolic
/// links are not resolved (use [`std::fs::canonicalize`] for that). A `..` at the start of a
/// relative path is kept, a `..` right after the root is dropped, and a path that cleans to
/// nothing becomes `.`.
///
/// # Arguments
///
/// - `path` - The path to clean.
///
/// # Returns
///
/// The cleaned path.
///
/// # Examples
///
/// ```
/// use helpers4::fs::normalize;
/// use std::path::{Path, PathBuf};
///
/// assert_eq!(normalize(Path::new("a/./b/../c")), PathBuf::from("a/c"));
/// assert_eq!(normalize(Path::new("../a")), PathBuf::from("../a"));
/// assert_eq!(normalize(Path::new("a/..")), PathBuf::from("."));
/// ```
#[must_use]
pub fn normalize(path: &Path) -> PathBuf {
    let cleaned: PathBuf = parts(path).into_iter().collect();
    if cleaned.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        cleaned
    }
}

#[cfg(test)]
#[path = "normalize.test.rs"]
mod tests;

#[cfg(test)]
#[path = "normalize.spec.rs"]
mod spec;
