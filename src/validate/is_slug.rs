// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Checks whether `s` has the shape of an ASCII URL slug: non-empty, made only of lowercase
/// ASCII letters, digits and hyphens, with no leading, trailing or doubled hyphen.
///
/// This is ASCII-only by design, even though [`slugify`](crate::string::slugify) keeps Unicode
/// letters (`slugify("café")` is `"café"`, which `is_slug` rejects): a slug meant to go
/// unescaped in a URL path is conventionally ASCII. For ASCII input, `is_slug(&slugify(s))` is
/// `true` whenever `slugify(s)` is not empty.
///
/// # Arguments
///
/// - `s` - The text to check.
///
/// # Returns
///
/// `true` when `s` already has the shape of a slug.
///
/// # Examples
///
/// ```
/// use helpers4::validate::is_slug;
///
/// assert!(is_slug("hello-world"));
/// assert!(is_slug("v2"));
/// assert!(!is_slug("Hello-World"));
/// assert!(!is_slug("-leading"));
/// assert!(!is_slug("double--hyphen"));
/// assert!(!is_slug(""));
/// ```
#[must_use]
pub fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

#[cfg(test)]
#[path = "is_slug.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_slug.spec.rs"]
mod spec;
