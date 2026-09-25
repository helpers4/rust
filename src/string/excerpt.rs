// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::squish::squish;
use super::truncate::truncate;

/// Derives a short, readable excerpt from a longer text — for a card, header or preview where a
/// full paragraph does not fit.
///
/// Unlike [`truncate`] (a mechanical cut at exactly `max_chars`), this prefers to cut at the end
/// of a whole sentence (`.`, `!` or `?`) when one fits within `max_chars`, even if that leaves
/// the result shorter than the limit. Only when no sentence fits does it fall back to the last
/// whole word before the limit — it never returns partial-word text.
///
/// Internal whitespace (line breaks, repeated spaces) is collapsed to single spaces first (see
/// [`squish`]).
///
/// Known limitation: the sentence-boundary check is a simple heuristic (punctuation followed by
/// whitespace or the end of the text) — it does not special-case abbreviations (`"Mr."`) or
/// decimal numbers (`"3.14"`), which can be misread as a sentence end.
///
/// # Arguments
///
/// - `text` - The text to excerpt.
/// - `max_chars` - The maximum length of the result, `ellipsis` included.
/// - `ellipsis` - Appended only when falling back to a word-boundary cut (a sentence-boundary
///   cut never needs one).
///
/// # Returns
///
/// The original text, squished, unchanged if already within `max_chars`; otherwise a shortened
/// version.
///
/// # Examples
///
/// ```
/// use helpers4::string::excerpt;
///
/// assert_eq!(excerpt("A short game about ducks.", 200, "…"), "A short game about ducks.");
///
/// assert_eq!(
///     excerpt("Build the biggest, best theme park ever seen. Can you make money?", 47, "…"),
///     "Build the biggest, best theme park ever seen.",
/// );
///
/// assert_eq!(
///     excerpt("This description has no punctuation at all so it must cut on a word", 30, "…"),
///     "This description has no…",
/// );
/// ```
#[must_use]
pub fn excerpt(text: &str, max_chars: usize, ellipsis: &str) -> String {
    let collapsed = squish(text);
    let chars: Vec<char> = collapsed.chars().collect();
    if chars.len() <= max_chars {
        return collapsed;
    }

    let window = &chars[..max_chars];
    if let Some(sentence_end) = find_sentence_end(window) {
        return window[..=sentence_end].iter().collect();
    }

    let ellipsis_len = ellipsis.chars().count();
    let budget = max_chars.saturating_sub(ellipsis_len).min(window.len());
    if let Some(cut) = word_boundary_cut(&window[..budget]) {
        let mut out: String = window[..cut].iter().collect();
        let trimmed_len = out.trim_end().len();
        out.truncate(trimmed_len);
        out.push_str(ellipsis);
        return out;
    }

    // No word boundary within budget (one giant unbroken run of characters): delegate to
    // `truncate` for a char-safe cut instead of slicing mid-character.
    truncate(&collapsed, max_chars, ellipsis)
}

/// The char index, within `window`, of the last `.`, `!` or `?` that is followed by whitespace
/// or the end of `window` — the last one, so the excerpt keeps as much of the window as fits.
fn find_sentence_end(window: &[char]) -> Option<usize> {
    window
        .iter()
        .enumerate()
        .filter(|&(i, &c)| {
            matches!(c, '.' | '!' | '?') && window.get(i + 1).is_none_or(|n| n.is_whitespace())
        })
        .map(|(i, _)| i)
        .next_back()
}

/// The char index of the last space in `window`, if any.
///
/// Never `Some(0)` in practice: `window` always starts at `collapsed`'s first character, and
/// `squish` already trimmed any leading whitespace off `collapsed` (or `excerpt` returned
/// early, on an all-whitespace `text`, before this is ever called).
fn word_boundary_cut(window: &[char]) -> Option<usize> {
    window.iter().rposition(|&c| c == ' ')
}

#[cfg(test)]
#[path = "excerpt.test.rs"]
mod tests;

#[cfg(test)]
#[path = "excerpt.spec.rs"]
mod spec;
