// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A pattern read by [`parse_pattern`]: a glob, or a regular expression kept as text for the
/// regex engine of your choice (this crate has none).
///
/// # Examples
///
/// ```
/// use helpers4::string::{parse_pattern, Pattern};
///
/// assert_eq!(parse_pattern("SF Pro*"), Pattern::Glob("SF Pro*".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Pattern {
    /// A glob, to use with [`match_glob`](super::match_glob).
    Glob(String),
    /// A regular expression written `/source/flags`.
    RegExp {
        /// The source, without the slashes.
        source: String,
        /// The flags (any of `d`, `g`, `i`, `m`, `s`, `u`, `v`, `y`), possibly empty.
        flags: String,
    },
}

/// Reads a setting that is either a regular expression or a glob: `/source/flags` is a regular
/// expression when every flag is valid (`d`, `g`, `i`, `m`, `s`, `u`, `v`, `y`), anything else is
/// a glob. For font substitution rules, ignore lists, filters. Note that `/usr/lib/img` is the
/// regular expression `usr/lib` with the flags `img`: for paths, build a [`Pattern::Glob`]
/// yourself.
///
/// The regular expression is returned as text, since this crate has no regex engine; the source
/// is not validated.
///
/// # Arguments
///
/// - `pattern` - A `/source/flags` regular expression or a glob.
///
/// # Returns
///
/// The [`Pattern`].
///
/// # Examples
///
/// ```
/// use helpers4::string::{parse_pattern, Pattern};
///
/// assert_eq!(
///     parse_pattern("/^SF Pro/i"),
///     Pattern::RegExp { source: "^SF Pro".to_string(), flags: "i".to_string() }
/// );
/// assert_eq!(parse_pattern("/src/*.ts/x"), Pattern::Glob("/src/*.ts/x".to_string()));
/// ```
#[must_use]
pub fn parse_pattern(pattern: &str) -> Pattern {
    pattern
        .strip_prefix('/')
        .and_then(|rest| rest.rsplit_once('/'))
        .filter(|(source, flags)| {
            !source.is_empty() && flags.chars().all(|c| "dgimsuvy".contains(c))
        })
        .map_or_else(
            || Pattern::Glob(pattern.to_string()),
            |(source, flags)| Pattern::RegExp {
                source: source.to_string(),
                flags: flags.to_string(),
            },
        )
}

#[cfg(test)]
#[path = "parse_pattern.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parse_pattern.spec.rs"]
mod spec;
