// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The text shown in place of a redacted secret.
pub const REDACTED: &str = "[REDACTED]";

/// Replaces every occurrence of each of `secrets` in `text` with `[REDACTED]`.
///
/// Meant for text that is about to be logged or shown (a command line, an HTTP dump, an error
/// message) when you know the secret values. Longer secrets are replaced first, so a secret that
/// contains another one is hidden whole. Empty secrets are ignored. Only exact occurrences are
/// found: a secret that was transformed (encoded, split over lines) is not.
///
/// # Arguments
///
/// - `text` - The text to clean.
/// - `secrets` - The values to hide.
///
/// # Returns
///
/// The text with every occurrence replaced.
///
/// # Examples
///
/// ```
/// use helpers4::secret::redact;
///
/// let line = "curl -H 'Authorization: Bearer abc123' https://x.org?key=abc123";
/// assert_eq!(
///     redact(line, &["abc123"]),
///     "curl -H 'Authorization: Bearer [REDACTED]' https://x.org?key=[REDACTED]"
/// );
/// ```
#[must_use]
pub fn redact(text: &str, secrets: &[&str]) -> String {
    let mut secrets: Vec<&str> = secrets.iter().copied().filter(|s| !s.is_empty()).collect();
    secrets.sort_by_key(|s| std::cmp::Reverse(s.len()));
    let mut out = text.to_string();
    for secret in secrets {
        out = out.replace(secret, REDACTED);
    }
    out
}

#[cfg(test)]
#[path = "redact.test.rs"]
mod tests;

#[cfg(test)]
#[path = "redact.spec.rs"]
mod spec;
