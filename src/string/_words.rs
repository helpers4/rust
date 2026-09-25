// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Word splitting shared by the case-conversion helpers and the public `words` (which needs the
//! same boundaries but the original casing, not lowercased). Not re-exported.

/// Byte ranges of each word in `s`.
///
/// A word ends at any non-alphanumeric character, before an uppercase letter that follows a
/// lowercase letter or a digit (`userName`, `v2Beta`), and before the last capital of an
/// acronym run (`HTMLParser` gives `html`, `parser`).
pub(crate) fn boundaries(s: &str) -> Vec<(usize, usize)> {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (i, &(pos, c)) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            if let Some(word_start) = start.take() {
                out.push((word_start, pos));
            }
            continue;
        }
        if c.is_uppercase() {
            if let Some(word_start) = start {
                let prev = chars[i - 1].1;
                let next_is_lower = chars.get(i + 1).is_some_and(|&(_, n)| n.is_lowercase());
                if prev.is_lowercase()
                    || prev.is_numeric()
                    || (prev.is_uppercase() && next_is_lower)
                {
                    out.push((word_start, pos));
                    start = None;
                }
            }
        }
        if start.is_none() {
            start = Some(pos);
        }
    }
    if let Some(word_start) = start {
        out.push((word_start, s.len()));
    }
    out
}

/// Splits `s` into lowercase words, using the same boundaries as [`boundaries`].
pub(crate) fn words(s: &str) -> Vec<String> {
    boundaries(s)
        .into_iter()
        .map(|(start, end)| s[start..end].to_lowercase())
        .collect()
}

#[cfg(test)]
#[path = "_words.test.rs"]
mod tests;
