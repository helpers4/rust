// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::match_glob;

/// A list of glob patterns with exclusions: a string matches when one pattern matches and no
/// exclusion does, like the lines of a `.gitignore` read the other way round.
///
/// Patterns use the `*` and `?` of [`match_glob`], which is case-sensitive. A pattern starting
/// with `!` is an exclusion; write `\!` for a pattern that really starts with `!`. The order does
/// not matter. With no pattern besides exclusions, everything else matches, and an empty set
/// matches everything (no filter).
///
/// Regular expressions need a dependency this crate does not have; use the `regex` crate for
/// those, or the TypeScript package's `createMatcher`.
///
/// # Examples
///
/// ```
/// use helpers4::string::GlobSet;
///
/// let screens = GlobSet::new(&["Screen *", "Home", "!* (old)"]);
/// assert!(screens.is_match("Screen 2"));
/// assert!(screens.is_match("Home"));
/// assert!(!screens.is_match("Screen 2 (old)")); // excluded
/// assert!(!screens.is_match("Settings")); // no pattern matches
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GlobSet {
    includes: Vec<String>,
    excludes: Vec<String>,
}

impl GlobSet {
    /// Builds a set from patterns, `!` marking the exclusions.
    ///
    /// # Arguments
    ///
    /// - `patterns` - The patterns, with `!` before an exclusion and `\!` for a literal `!`.
    ///
    /// # Returns
    ///
    /// The set.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::string::GlobSet;
    ///
    /// let set = GlobSet::new(&["*.fig", "!draft-*"]);
    /// assert!(set.is_match("a.fig"));
    /// assert!(!set.is_match("draft-a.fig"));
    /// ```
    #[must_use]
    pub fn new(patterns: &[&str]) -> Self {
        let mut set = Self::default();
        for &pattern in patterns {
            if let Some(excluded) = pattern.strip_prefix('!') {
                set.excludes.push(excluded.to_string());
            } else if let Some(literal) = pattern
                .strip_prefix('\\')
                .filter(|rest| rest.starts_with('!'))
            {
                set.includes.push(literal.to_string());
            } else {
                set.includes.push(pattern.to_string());
            }
        }
        set
    }

    /// Whether `text` matches the set.
    ///
    /// # Arguments
    ///
    /// - `text` - The string to test.
    ///
    /// # Returns
    ///
    /// `true` if one pattern matches and no exclusion does (or, with no pattern besides
    /// exclusions, if no exclusion does).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::string::GlobSet;
    ///
    /// assert!(GlobSet::new(&[]).is_match("anything"));
    /// assert!(GlobSet::new(&["!*.tmp"]).is_match("a.fig"));
    /// assert!(!GlobSet::new(&["!*.tmp"]).is_match("a.tmp"));
    /// ```
    #[must_use]
    pub fn is_match(&self, text: &str) -> bool {
        (self.includes.is_empty() || self.includes.iter().any(|p| match_glob(p, text)))
            && !self.excludes.iter().any(|p| match_glob(p, text))
    }
}

#[cfg(test)]
#[path = "glob_set.test.rs"]
mod tests;

#[cfg(test)]
#[path = "glob_set.spec.rs"]
mod spec;
