// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_prerelease::{check_identifiers, cmp_prerelease, number};
use super::ParseVersionError;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// A [Semantic Versioning 2.0.0](https://semver.org) version: `MAJOR.MINOR.PATCH`, optionally
/// with a pre-release (`-alpha.1`) and build metadata (`+sha.5114f85`).
///
/// [`Version::parse`] is strict semver; [`Version::parse_lenient`] also takes `v1.2`. Versions
/// order by semver precedence (a pre-release sorts before its release, numeric identifiers
/// compare as numbers); build metadata does not count for precedence
/// ([`precedence`](Self::precedence)) but breaks ties in `Ord` so that it stays consistent with
/// `Eq`.
///
/// # Examples
///
/// ```
/// use helpers4::version::Version;
///
/// let v = Version::parse("1.4.2-rc.1+build.7")?;
/// assert_eq!((v.major(), v.minor(), v.patch()), (1, 4, 2));
/// assert_eq!(v.pre(), Some("rc.1"));
/// assert!(v < Version::parse("1.4.2")?);
/// assert_eq!(v.bump_minor().to_string(), "1.5.0");
/// # Ok::<(), helpers4::version::ParseVersionError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<String>,
    build: Option<String>,
}

impl Version {
    /// A release version, with no pre-release or build metadata.
    ///
    /// # Arguments
    ///
    /// - `major` - The major version.
    /// - `minor` - The minor version.
    /// - `patch` - The patch version.
    ///
    /// # Returns
    ///
    /// The version `major.minor.patch`.
    #[must_use]
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
            build: None,
        }
    }

    /// Parses a strict semver version: exactly `MAJOR.MINOR.PATCH`, then optionally `-PRERELEASE`
    /// and `+BUILD`. No `v` prefix, no missing parts, no leading zeros in the numbers.
    ///
    /// # Arguments
    ///
    /// - `text` - The version to parse.
    ///
    /// # Errors
    ///
    /// A [`ParseVersionError`] naming what is wrong: an empty text, a missing or invalid number,
    /// a leading zero, or a bad pre-release or build identifier.
    pub fn parse(text: &str) -> Result<Self, ParseVersionError> {
        Self::parse_parts(text, false)
    }

    /// Parses a version the way people write it: surrounding whitespace and a leading `v` or `V`
    /// are ignored, and a missing minor or patch is `0` (`"v1.2"` is `1.2.0`, `"3"` is `3.0.0`).
    ///
    /// # Arguments
    ///
    /// - `text` - The version to parse.
    ///
    /// # Errors
    ///
    /// The same [`ParseVersionError`]s as [`Version::parse`], except that a missing minor or
    /// patch is not an error.
    pub fn parse_lenient(text: &str) -> Result<Self, ParseVersionError> {
        let text = text.trim();
        let text = text.strip_prefix(['v', 'V']).unwrap_or(text);
        Self::parse_parts(text, true)
    }

    fn parse_parts(text: &str, lenient: bool) -> Result<Self, ParseVersionError> {
        if text.is_empty() {
            return Err(ParseVersionError::Empty);
        }
        let (rest, build) = match text.split_once('+') {
            Some((rest, build)) => {
                check_identifiers(build, false)?;
                (rest, Some(build.to_string()))
            }
            None => (text, None),
        };
        let (core, pre) = match rest.split_once('-') {
            Some((core, pre)) => {
                check_identifiers(pre, true)?;
                (core, Some(pre.to_string()))
            }
            None => (rest, None),
        };
        let mut parts = core.splitn(3, '.');
        let major = number("major", parts.next().unwrap_or(""))?;
        let minor = match parts.next() {
            Some(minor) => number("minor", minor)?,
            None if lenient => 0,
            None => return Err(ParseVersionError::MissingComponent { component: "minor" }),
        };
        let patch = match parts.next() {
            Some(patch) => number("patch", patch)?,
            None if lenient => 0,
            None => return Err(ParseVersionError::MissingComponent { component: "patch" }),
        };
        Ok(Self {
            major,
            minor,
            patch,
            pre,
            build,
        })
    }

    /// The major version.
    ///
    /// # Returns
    ///
    /// The first number.
    #[must_use]
    pub fn major(&self) -> u64 {
        self.major
    }

    /// The minor version.
    ///
    /// # Returns
    ///
    /// The second number.
    #[must_use]
    pub fn minor(&self) -> u64 {
        self.minor
    }

    /// The patch version.
    ///
    /// # Returns
    ///
    /// The third number.
    #[must_use]
    pub fn patch(&self) -> u64 {
        self.patch
    }

    /// The pre-release identifiers, without the leading `-`.
    ///
    /// # Returns
    ///
    /// For instance `Some("alpha.1")`, or `None` for a release.
    #[must_use]
    pub fn pre(&self) -> Option<&str> {
        self.pre.as_deref()
    }

    /// The build metadata, without the leading `+`.
    ///
    /// # Returns
    ///
    /// For instance `Some("sha.5114f85")`, or `None`.
    #[must_use]
    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    /// Whether this is a pre-release (it has a `-...` part).
    ///
    /// # Returns
    ///
    /// `true` for `1.0.0-alpha`, `false` for `1.0.0`.
    #[must_use]
    pub fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }

    /// The next major version: `1.4.2-rc.1` gives `2.0.0`.
    ///
    /// # Returns
    ///
    /// A release with the major version incremented and the rest reset. It saturates at
    /// `u64::MAX` instead of overflowing.
    #[must_use]
    pub fn bump_major(&self) -> Self {
        Self::new(self.major.saturating_add(1), 0, 0)
    }

    /// The next minor version: `1.4.2-rc.1` gives `1.5.0`.
    ///
    /// # Returns
    ///
    /// A release with the minor version incremented and the patch reset. It saturates at
    /// `u64::MAX` instead of overflowing.
    #[must_use]
    pub fn bump_minor(&self) -> Self {
        Self::new(self.major, self.minor.saturating_add(1), 0)
    }

    /// The next patch version: `1.4.2-rc.1` gives `1.4.3`.
    ///
    /// # Returns
    ///
    /// A release with the patch version incremented. It saturates at `u64::MAX` instead of
    /// overflowing.
    #[must_use]
    pub fn bump_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch.saturating_add(1))
    }

    /// Compares two versions by semver precedence: build metadata is ignored, so `1.0.0+a` and
    /// `1.0.0+b` are equal here.
    ///
    /// # Arguments
    ///
    /// - `other` - The version to compare with.
    ///
    /// # Returns
    ///
    /// How this version orders relative to `other`.
    #[must_use]
    pub fn precedence(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
            .then_with(|| cmp_prerelease(self.pre(), other.pre()))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence(other)
            .then_with(|| self.build.cmp(&other.build))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre) = &self.pre {
            out.push('-');
            out.push_str(pre);
        }
        if let Some(build) = &self.build {
            out.push('+');
            out.push_str(build);
        }
        f.write_str(&out)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "semantic_version.test.rs"]
mod tests;

#[cfg(test)]
#[path = "semantic_version.spec.rs"]
mod spec;
