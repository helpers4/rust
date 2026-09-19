// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Word splitting shared by the case-conversion helpers. Not re-exported.

fn flush(current: &mut String, out: &mut Vec<String>) {
    if !current.is_empty() {
        out.push(std::mem::take(current));
    }
}

/// Splits `s` into lowercase words.
///
/// A word ends at any non-alphanumeric character, before an uppercase letter that follows a
/// lowercase letter or a digit (`userName`, `v2Beta`), and before the last capital of an
/// acronym run (`HTMLParser` gives `html`, `parser`).
pub(crate) fn words(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut current = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            flush(&mut current, &mut out);
            continue;
        }
        if c.is_uppercase() && !current.is_empty() {
            let prev = chars[i - 1];
            let next_is_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            if prev.is_lowercase() || prev.is_numeric() || (prev.is_uppercase() && next_is_lower) {
                flush(&mut current, &mut out);
            }
        }
        current.extend(c.to_lowercase());
    }
    flush(&mut current, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<String> {
        words(s)
    }

    #[test]
    fn empty_and_separator_only_inputs_have_no_words() {
        assert!(w("").is_empty());
        assert!(w(" -_ ").is_empty());
    }

    #[test]
    fn splits_on_non_alphanumeric() {
        assert_eq!(
            w("hello-world_foo bar.baz"),
            ["hello", "world", "foo", "bar", "baz"]
        );
    }

    #[test]
    fn splits_lower_to_upper_and_digit_to_upper() {
        assert_eq!(w("userName"), ["user", "name"]);
        assert_eq!(w("v2Beta"), ["v2", "beta"]);
    }

    #[test]
    fn keeps_acronym_runs_together_until_the_last_capital() {
        assert_eq!(w("HTMLParser"), ["html", "parser"]);
        assert_eq!(w("userID"), ["user", "id"]);
        assert_eq!(w("ABC"), ["abc"]);
    }

    #[test]
    fn does_not_split_letters_from_trailing_digits() {
        assert_eq!(w("version2"), ["version2"]);
    }

    #[test]
    fn handles_caseless_scripts() {
        assert_eq!(w("日本語 テスト"), ["日本語", "テスト"]);
    }
}
