// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{ParseVersionError, Version, VersionReq};

/// Whether the version `version` satisfies the requirement `requirement`, both given as text.
///
/// A shortcut for [`Version::parse_lenient`], [`VersionReq::parse`] and [`VersionReq::matches`]:
/// use those directly to check many versions against one requirement.
///
/// # Arguments
///
/// - `version` - The version to check, such as `"1.4.2"`.
/// - `requirement` - The requirement, such as `"^1.2"` (see [`VersionReq`] for the syntax).
///
/// # Errors
///
/// The [`ParseVersionError`] of whichever string does not parse.
///
/// # Examples
///
/// ```
/// use helpers4::version::satisfies;
///
/// assert!(satisfies("1.4.2", "^1.2")?);
/// assert!(!satisfies("2.0.0", "^1.2")?);
/// assert!(satisfies("0.3.1", ">=0.3, <0.4")?);
/// # Ok::<(), helpers4::version::ParseVersionError>(())
/// ```
pub fn satisfies(version: &str, requirement: &str) -> Result<bool, ParseVersionError> {
    let version = Version::parse_lenient(version)?;
    Ok(VersionReq::parse(requirement)?.matches(&version))
}

#[cfg(test)]
#[path = "satisfies.test.rs"]
mod tests;
