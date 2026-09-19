// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Line-level dotenv parsing and formatting shared by the public helpers. Not re-exported.
//!
//! The format is line based: `KEY=value`, an optional `export ` prefix, `#` comment lines, and
//! values that are bare, `"double quoted"` (with `\n \r \t \" \\` escapes) or `'single quoted'`
//! (literal). A bare value ends at the first `#` preceded by whitespace. Quoted values cannot
//! span lines.

/// Whether `key` is a valid variable name: `[A-Za-z_][A-Za-z0-9_]*`.
pub(crate) fn is_valid_key(key: &str) -> bool {
    let mut chars = key.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Parses one line into `(key, value)`; `None` for blank lines, comments and anything that is
/// not a valid assignment.
pub(crate) fn parse_line(line: &str) -> Option<(&str, String)> {
    let trimmed = line.trim();
    if trimmed.starts_with('#') {
        return None;
    }
    let body = trimmed
        .strip_prefix("export ")
        .map_or(trimmed, str::trim_start);
    let (key, raw) = body.split_once('=')?;
    let key = key.trim();
    is_valid_key(key).then(|| (key, unquote(raw.trim())))
}

fn unquote(raw: &str) -> String {
    if let Some(rest) = raw.strip_prefix('"') {
        if let Some(value) = unescape_double_quoted(rest) {
            return value;
        }
    } else if let Some(rest) = raw.strip_prefix('\'') {
        if let Some(end) = rest.find('\'') {
            return rest[..end].to_string();
        }
    }
    strip_inline_comment(raw).to_string()
}

/// Reads up to the closing quote; `None` when the quote is never closed.
fn unescape_double_quoted(rest: &str) -> Option<String> {
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some(escaped @ ('"' | '\\')) => out.push(escaped),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            },
            other => out.push(other),
        }
    }
    None
}

fn strip_inline_comment(raw: &str) -> &str {
    match raw.find(" #").into_iter().chain(raw.find("\t#")).min() {
        Some(index) => raw[..index].trim_end(),
        None => raw,
    }
}

#[cfg(test)]
#[path = "_line.test.rs"]
mod tests;
