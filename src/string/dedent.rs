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
///
/// # Since
///
/// next
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
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn strips_common_indent_and_outer_blank_lines() {
        assert_eq!(dedent("\n    Hello\n      World\n"), "Hello\n  World");
    }

    #[test]
    fn works_without_outer_blank_lines() {
        assert_eq!(dedent("  a\n  b"), "a\nb");
    }

    #[test]
    fn blank_lines_do_not_affect_the_indent() {
        assert_eq!(dedent("  a\n\n  b"), "a\n\nb");
    }

    #[test]
    fn single_blank_line_is_kept_as_is() {
        assert_eq!(dedent(""), "");
        assert_eq!(dedent("   "), "   ");
    }

    #[test]
    fn all_blank_lines_have_no_indent_to_strip() {
        assert_eq!(dedent("\n\n"), "");
    }

    #[test]
    fn unindented_input_is_unchanged() {
        assert_eq!(dedent("a\nb"), "a\nb");
    }

    #[test]
    fn counts_multibyte_whitespace_as_one_indent_unit() {
        assert_eq!(dedent("\u{a0}a\n b"), "a\nb");
    }

    proptest! {
        #[test]
        fn removes_exactly_the_shared_indent(
            words in prop::collection::vec("[a-z]{1,5}", 1..5),
            n in 0usize..6,
        ) {
            let pad = " ".repeat(n);
            let indented = words.iter().map(|w| format!("{pad}{w}")).collect::<Vec<_>>().join("\n");
            prop_assert_eq!(dedent(&indented), words.join("\n"));
        }
    }
}
