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
#[path = "_words.test.rs"]
mod tests;
