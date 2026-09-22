// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Prefixes every non-blank line of `s` with `prefix`.
///
/// Blank lines (empty or whitespace only) are left untouched, so no trailing whitespace is
/// introduced. Lines are split on `'\n'` only, and a trailing newline is preserved. It is the
/// inverse of [`dedent`](super::dedent) for text indented with a fixed prefix.
///
/// # Arguments
///
/// - `s` - The text to indent.
/// - `prefix` - The text to prepend to every non-blank line.
///
/// # Examples
///
/// ```
/// use helpers4::string::indent;
///
/// assert_eq!(indent("a\n\nb", "  "), "  a\n\n  b");
/// assert_eq!(indent("line\n", "> "), "> line\n");
/// ```
#[must_use]
pub fn indent(s: &str, prefix: &str) -> String {
    s.split('\n')
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "indent.test.rs"]
mod tests;

#[cfg(test)]
#[path = "indent.spec.rs"]
mod spec;
