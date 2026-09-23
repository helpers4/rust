// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A fenced code block for `code`, tagged with `language`.
///
/// The fence is at least three backticks and one longer than the longest run of backticks in the
/// code, so code that itself contains a fence stays inside the block. The result ends with a
/// newline and the code keeps its own line breaks (a single trailing newline is not doubled).
/// Whitespace and backticks are removed from `language`, which must be a single word (`rust`,
/// `sh`, `json`, or empty for none).
///
/// # Arguments
///
/// - `code` - The code to show.
/// - `language` - The language tag for syntax highlighting, or `""`.
///
/// # Returns
///
/// The fenced block, ending with a newline.
///
/// # Examples
///
/// ```
/// use helpers4::markdown::code_block;
///
/// assert_eq!(code_block("let x = 1;", "rust"), "```rust\nlet x = 1;\n```\n");
/// assert_eq!(code_block("```\nnested\n```", ""), "````\n```\nnested\n```\n````\n");
/// ```
#[must_use]
pub fn code_block(code: &str, language: &str) -> String {
    let longest = code.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest.max(2) + 1);
    let language: String = language
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '`')
        .collect();
    if code.is_empty() {
        return format!("{fence}{language}\n{fence}\n");
    }
    let body = code.strip_suffix('\n').unwrap_or(code);
    format!("{fence}{language}\n{body}\n{fence}\n")
}

#[cfg(test)]
#[path = "code_block.test.rs"]
mod tests;

#[cfg(test)]
#[path = "code_block.spec.rs"]
mod spec;
