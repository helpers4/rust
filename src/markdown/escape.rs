// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Escapes `text` so that Markdown shows it literally instead of interpreting it.
///
/// A backslash is put before every character that can start emphasis, code, a link, a tag, a table
/// cell or a strikethrough (`` \ ` * _ [ ] < > | ~ & ``), and before the characters that only
/// matter at the start of a line: `#`, `+`, `-`, `=`, and the `.` or `)` of `1.` / `1)` list
/// markers. Escaping more than strictly needed is harmless (the Markdown spec accepts a backslash before
/// any ASCII punctuation) and keeps this predictable. Use it for text you do not control that you
/// put into a Markdown document, such as a user name or an error message.
///
/// # Arguments
///
/// - `text` - The text to escape, on one or several lines.
///
/// # Returns
///
/// The escaped text.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::escape;
///
/// assert_eq!(escape("*not bold* and [not a link](x)"), "\\*not bold\\* and \\[not a link\\](x)");
/// assert_eq!(escape("# not a heading"), "\\# not a heading");
/// assert_eq!(escape("1. not a list"), "1\\. not a list");
/// ```
#[must_use]
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 8);
    for line in text.split_inclusive('\n') {
        let chars: Vec<char> = line.chars().collect();
        let mut i = chars.iter().take_while(|c| **c == ' ').count();
        out.extend(&chars[..i]);
        if i < chars.len() {
            match chars[i] {
                '#' | '+' | '-' | '=' => out.push('\\'),
                c if c.is_ascii_digit() => {
                    let digits_end = chars[i..]
                        .iter()
                        .position(|d| !d.is_ascii_digit())
                        .map_or(chars.len(), |offset| i + offset);
                    out.extend(&chars[i..digits_end]);
                    i = digits_end;
                    if matches!(chars.get(i), Some('.' | ')')) {
                        out.push('\\');
                    }
                }
                _ => {}
            }
        }
        for &c in &chars[i..] {
            if matches!(
                c,
                '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '|' | '~' | '&'
            ) {
                out.push('\\');
            }
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
#[path = "escape.test.rs"]
mod tests;

#[cfg(test)]
#[path = "escape.spec.rs"]
mod spec;
