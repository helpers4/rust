// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_line::parse_line;

/// Parses dotenv `content` into `(key, value)` pairs, in file order.
///
/// Blank lines, `#` comments and lines that are not valid `KEY=value` assignments are skipped.
/// Supported: an optional `export ` prefix, bare values (a `#` after whitespace starts a
/// comment), `"double quoted"` values with `\n \r \t \" \\` escapes and `'single quoted'`
/// literals. Values are single-line. A key assigned twice appears twice.
///
/// # Examples
///
/// ```
/// use helpers4::env::parse;
///
/// let vars = parse("# comment\nHOST=localhost\nexport NAME=\"my app\"  # trailing\n");
/// assert_eq!(
///     vars,
///     vec![
///         ("HOST".to_string(), "localhost".to_string()),
///         ("NAME".to_string(), "my app".to_string()),
///     ]
/// );
/// ```
pub fn parse(content: &str) -> Vec<(String, String)> {
    content
        .lines()
        .filter_map(parse_line)
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

#[cfg(test)]
#[path = "parse.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parse.spec.rs"]
mod spec;
