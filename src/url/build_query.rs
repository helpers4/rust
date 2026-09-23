// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::percent_encode;

/// Builds a query string (`a=1&b=two`) from key-value pairs, percent-encoding both sides.
///
/// Pairs keep their order and repeated keys are kept. A space becomes `%20` (not `+`), which every
/// URL parser reads back correctly. The result has no leading `?`. It is the inverse of
/// [`parse_query`](super::parse_query).
///
/// # Arguments
///
/// - `pairs` - The keys and values, anything that iterates over `(key, value)` of string-likes.
///
/// # Returns
///
/// The encoded query string, empty when there are no pairs.
///
/// # Examples
///
/// ```
/// use helpers4::url::build_query;
///
/// assert_eq!(build_query([("q", "rust lang"), ("page", "2")]), "q=rust%20lang&page=2");
/// assert_eq!(build_query(Vec::<(&str, &str)>::new()), "");
/// ```
#[must_use]
pub fn build_query<I, K, V>(pairs: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    pairs
        .into_iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                percent_encode(key.as_ref()),
                percent_encode(value.as_ref())
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

#[cfg(test)]
#[path = "build_query.test.rs"]
mod tests;
