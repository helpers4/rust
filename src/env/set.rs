// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_line::{format_value, is_valid_key, parse_line};
use super::error::InvalidKeyError;

/// Sets `key` to `value` in dotenv `content` and returns the new content.
///
/// The first existing assignment of `key` is replaced in place and any later ones are dropped;
/// when there is none, a new line is appended. Every other line (comments, blank lines, other
/// variables) is left untouched, and the replaced line keeps its line ending. `value` is quoted
/// and escaped only when needed, so [`get`](super::get) reads it back exactly.
///
/// # Errors
///
/// Returns [`InvalidKeyError`] when `key` is not `[A-Za-z_][A-Za-z0-9_]*`.
///
/// # Examples
///
/// ```
/// use helpers4::env::set;
///
/// let updated = set("# config\nHOST=old\nPORT=80\n", "HOST", "example.com")?;
/// assert_eq!(updated, "# config\nHOST=example.com\nPORT=80\n");
///
/// assert_eq!(set("A=1\n", "B", "two words")?, "A=1\nB=\"two words\"\n");
/// # Ok::<(), helpers4::env::InvalidKeyError>(())
/// ```
pub fn set(content: &str, key: &str, value: &str) -> Result<String, InvalidKeyError> {
    if !is_valid_key(key) {
        return Err(InvalidKeyError::new(key));
    }
    let assignment = format!("{key}={}", format_value(value));
    let mut out = String::with_capacity(content.len() + assignment.len() + 1);
    let mut replaced = false;
    for line in content.split_inclusive('\n') {
        match parse_line(line) {
            Some((k, _)) if k == key => {
                if !replaced {
                    out.push_str(&assignment);
                    out.push_str(line_ending(line));
                    replaced = true;
                }
            }
            _ => out.push_str(line),
        }
    }
    if !replaced {
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&assignment);
        out.push('\n');
    }
    Ok(out)
}

fn line_ending(line: &str) -> &'static str {
    if line.ends_with("\r\n") {
        "\r\n"
    } else if line.ends_with('\n') {
        "\n"
    } else {
        ""
    }
}

#[cfg(test)]
#[path = "set.test.rs"]
mod tests;

#[cfg(test)]
#[path = "set.spec.rs"]
mod spec;
