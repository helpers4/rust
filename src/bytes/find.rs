// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The index of the first occurrence of `needle` in `haystack`.
///
/// The byte-slice counterpart of [`str::find`]: the standard library has no subslice search.
///
/// # Arguments
///
/// - `haystack` - The bytes to search in.
/// - `needle` - The bytes to look for.
///
/// # Returns
///
/// The starting index of the first match, `Some(0)` for an empty `needle`, or `None` when there
/// is no match.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::find;
///
/// assert_eq!(find(b"hello world", b"o w"), Some(4));
/// assert_eq!(find(b"hello", b"xyz"), None);
/// assert_eq!(find(b"hello", b""), Some(0));
/// ```
#[must_use]
pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
#[path = "find.test.rs"]
mod tests;

#[cfg(test)]
#[path = "find.spec.rs"]
mod spec;
