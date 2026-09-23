// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{ParseVersionError, Version};
use std::cmp::Ordering;

/// Compares two version strings by semver precedence.
///
/// Both are read leniently (see [`Version::parse_lenient`]: a `v` prefix and missing minor or
/// patch are fine), and build metadata is ignored.
///
/// # Arguments
///
/// - `a` - The first version.
/// - `b` - The second version.
///
/// # Errors
///
/// The [`ParseVersionError`] of whichever string does not parse (the first one if both fail).
///
/// # Examples
///
/// ```
/// use helpers4::version::compare;
/// use std::cmp::Ordering;
///
/// assert_eq!(compare("1.10.0", "1.9.0")?, Ordering::Greater);
/// assert_eq!(compare("v2", "2.0.0")?, Ordering::Equal);
/// assert_eq!(compare("1.0.0-rc.1", "1.0.0")?, Ordering::Less);
/// # Ok::<(), helpers4::version::ParseVersionError>(())
/// ```
pub fn compare(a: &str, b: &str) -> Result<Ordering, ParseVersionError> {
    Ok(Version::parse_lenient(a)?.precedence(&Version::parse_lenient(b)?))
}

#[cfg(test)]
#[path = "compare.test.rs"]
mod tests;
