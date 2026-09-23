// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a version or a version requirement could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseVersionError {
    /// The text is empty or only whitespace.
    Empty,
    /// A part is missing: `"1.2"` has no patch.
    MissingComponent {
        /// `"minor"` or `"patch"`.
        component: &'static str,
    },
    /// A part is not a number (or does not fit a `u64`).
    InvalidNumber {
        /// `"major"`, `"minor"` or `"patch"`.
        component: &'static str,
    },
    /// A number has a leading zero (`"01"`), which semver forbids.
    LeadingZero {
        /// `"major"`, `"minor"`, `"patch"` or `"prerelease"` (a numeric identifier).
        component: &'static str,
    },
    /// A pre-release or build identifier is empty or has a character outside `[0-9A-Za-z-]`.
    InvalidIdentifier,
    /// A requirement comparator is malformed: unknown operator, a wildcard where it is not
    /// allowed, a pre-release on a partial version, or build metadata.
    InvalidComparator,
}

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty version"),
            Self::MissingComponent { component } => write!(f, "missing {component} version"),
            Self::InvalidNumber { component } => write!(f, "invalid {component} version number"),
            Self::LeadingZero { component } => write!(f, "leading zero in the {component} version"),
            Self::InvalidIdentifier => write!(f, "invalid pre-release or build identifier"),
            Self::InvalidComparator => write!(f, "invalid version requirement"),
        }
    }
}

impl std::error::Error for ParseVersionError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
