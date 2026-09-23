// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{PercentDecodeError, percent_decode};

/// Parses a query string (`a=1&b=two`) into its key-value pairs, decoded, in order.
///
/// A leading `?` is ignored. Pairs are separated by `&`; empty pairs are skipped; a pair without
/// `=` has an empty value; only the first `=` splits key from value. Repeated keys are all kept.
/// Both keys and values are decoded the way HTML forms are: `+` is a space and `%XX` is a byte.
///
/// # Arguments
///
/// - `query` - The query string, with or without the leading `?`.
///
/// # Errors
///
/// A [`PercentDecodeError`] for an invalid escape in a key or a value (its `index` is a byte
/// offset within that key or value).
///
/// # Examples
///
/// ```
/// use helpers4::url::parse_query;
///
/// assert_eq!(
///     parse_query("?q=rust+lang&page=2&flag")?,
///     vec![
///         ("q".to_string(), "rust lang".to_string()),
///         ("page".to_string(), "2".to_string()),
///         ("flag".to_string(), String::new()),
///     ]
/// );
/// # Ok::<(), helpers4::url::PercentDecodeError>(())
/// ```
pub fn parse_query(query: &str) -> Result<Vec<(String, String)>, PercentDecodeError> {
    let query = query.strip_prefix('?').unwrap_or(query);
    let mut pairs = Vec::new();
    for part in query.split('&').filter(|part| !part.is_empty()) {
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        pairs.push((form_decode(key)?, form_decode(value)?));
    }
    Ok(pairs)
}

fn form_decode(component: &str) -> Result<String, PercentDecodeError> {
    percent_decode(&component.replace('+', " ")).map(std::borrow::Cow::into_owned)
}

#[cfg(test)]
#[path = "parse_query.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parse_query.spec.rs"]
mod spec;
