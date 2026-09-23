// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Pre-release and number handling shared by `Version` and `VersionReq`. Not re-exported.

use super::ParseVersionError;
use std::cmp::Ordering;

/// Parses one numeric part: digits only, no leading zero, fits a `u64`.
pub(crate) fn number(component: &'static str, text: &str) -> Result<u64, ParseVersionError> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ParseVersionError::InvalidNumber { component });
    }
    if text.len() > 1 && text.starts_with('0') {
        return Err(ParseVersionError::LeadingZero { component });
    }
    text.parse()
        .map_err(|_| ParseVersionError::InvalidNumber { component })
}

/// Checks a dot-separated list of identifiers made of `[0-9A-Za-z-]`. In a pre-release, a numeric
/// identifier must not have a leading zero.
pub(crate) fn check_identifiers(text: &str, prerelease: bool) -> Result<(), ParseVersionError> {
    for identifier in text.split('.') {
        if identifier.is_empty()
            || !identifier
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(ParseVersionError::InvalidIdentifier);
        }
        if prerelease
            && is_numeric(identifier)
            && identifier.len() > 1
            && identifier.starts_with('0')
        {
            return Err(ParseVersionError::LeadingZero {
                component: "prerelease",
            });
        }
    }
    Ok(())
}

fn is_numeric(identifier: &str) -> bool {
    identifier.bytes().all(|b| b.is_ascii_digit())
}

/// semver precedence of two pre-release fields: a version without one is greater, and otherwise
/// identifiers are compared one by one (numbers numerically and below text, text in ASCII order,
/// a shorter list below a longer one that starts the same).
pub(crate) fn cmp_prerelease(a: Option<&str>, b: Option<&str>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(a), Some(b)) => {
            let mut left = a.split('.');
            let mut right = b.split('.');
            loop {
                match (left.next(), right.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(x), Some(y)) => {
                        let order = cmp_identifier(x, y);
                        if order != Ordering::Equal {
                            return order;
                        }
                    }
                }
            }
        }
    }
}

fn cmp_identifier(x: &str, y: &str) -> Ordering {
    match (is_numeric(x), is_numeric(y)) {
        // No leading zeros, so a longer number is a bigger one.
        (true, true) => x.len().cmp(&y.len()).then_with(|| x.cmp(y)),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => x.cmp(y),
    }
}

#[cfg(test)]
#[path = "_prerelease.test.rs"]
mod tests;
