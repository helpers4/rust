// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Strips the indentation shared by every non-blank line of `s`, and drops one leading and one
/// trailing blank line.
///
/// Lets a multi-line string literal be indented with the surrounding code without that
/// indentation leaking into the value. Indentation is counted in whitespace characters, and
/// lines are split on `'\n'` only (a `'\r'` stays on its line).
///
/// # Examples
///
/// ```
/// use helpers4::string::dedent;
///
/// assert_eq!(dedent("\n    Hello\n      World\n"), "Hello\n  World");
/// assert_eq!(dedent("  a\n  b"), "a\nb");
/// ```
#[must_use]
pub fn dedent(s: &str) -> String {
    let mut lines: Vec<&str> = s.split('\n').collect();
    if lines.len() > 1 && lines[0].trim().is_empty() {
        lines.remove(0);
    }
    if lines.len() > 1 && lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| line.chars().skip(min_indent).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "dedent.test.rs"]
mod tests;

#[cfg(test)]
#[path = "dedent.spec.rs"]
mod spec;
