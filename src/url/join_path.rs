// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Joins two pieces of a URL or path with exactly one `/` between them.
///
/// Trailing slashes of `base` and leading slashes of `segment` are trimmed first, so
/// `"a/"` + `"/b"`, `"a"` + `"b"` and `"a//"` + `"b"` all give `"a/b"`. Nothing is encoded or
/// resolved: use [`Url::join`](super::Url::join) to resolve a reference against a base URL, and
/// [`percent_encode`](super::percent_encode) for a segment that may contain reserved characters.
///
/// # Arguments
///
/// - `base` - The first part, such as `"https://api.example.com/v1/"`.
/// - `segment` - The part to append, such as `"/users"`.
///
/// # Returns
///
/// The two parts joined by a single `/`.
///
/// # Examples
///
/// ```
/// use helpers4::url::join_path;
///
/// assert_eq!(join_path("https://api.example.com/v1/", "/users"), "https://api.example.com/v1/users");
/// assert_eq!(join_path("a", "b"), "a/b");
/// ```
#[must_use]
pub fn join_path(base: &str, segment: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        segment.trim_start_matches('/')
    )
}

#[cfg(test)]
#[path = "join_path.test.rs"]
mod tests;

#[cfg(test)]
#[path = "join_path.spec.rs"]
mod spec;
