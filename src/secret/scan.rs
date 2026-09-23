// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{TokenKind, detect};

/// One credential found by [`scan`](super::scan).
///
/// # Examples
///
/// ```
/// use helpers4::secret::{scan, TokenKind};
///
/// let text = "key=AKIAIOSFODNN7EXAMPLE";
/// let finding = &scan(text)[0];
/// assert_eq!(finding.kind(), TokenKind::AwsAccessKey);
/// assert_eq!(&text[finding.start()..finding.end()], "AKIAIOSFODNN7EXAMPLE");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Finding {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Finding {
    /// The kind of credential.
    ///
    /// # Returns
    ///
    /// The recognized [`TokenKind`].
    #[must_use]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// The byte offset where the credential starts in the scanned text.
    ///
    /// # Returns
    ///
    /// The start, a character boundary.
    #[must_use]
    pub fn start(&self) -> usize {
        self.start
    }

    /// The byte offset just past the end of the credential.
    ///
    /// # Returns
    ///
    /// The end, a character boundary.
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }
}

/// Finds the well-known credentials in `text`.
///
/// The text is cut into words made of letters, digits, `_`, `-` and `.`, and each word is checked
/// with [`detect`](super::detect); a PEM private key header line (`-----BEGIN ... PRIVATE KEY-----`)
/// is found as a whole line. Quotes, `=`, `:`, spaces and other punctuation around a token are not
/// part of it. Like `detect` this is a format check: expect false negatives for formats it does
/// not know, and use it as a safety net, not as your only protection.
///
/// # Arguments
///
/// - `text` - The text to search, such as a file or a log.
///
/// # Returns
///
/// The findings in the order they appear, with their byte ranges in `text`.
///
/// # Examples
///
/// ```
/// use helpers4::secret::{scan, TokenKind};
///
/// let findings = scan("AWS_KEY=\"AKIAIOSFODNN7EXAMPLE\" # and nothing else");
/// assert_eq!(findings.len(), 1);
/// assert_eq!(findings[0].kind(), TokenKind::AwsAccessKey);
/// ```
#[must_use]
pub fn scan(text: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        if let Some(kind) = detect(trimmed).filter(|kind| *kind == TokenKind::PrivateKey) {
            let start = offset + line.find(trimmed).unwrap_or(0);
            findings.push(Finding {
                kind,
                start,
                end: start + trimmed.len(),
            });
        } else {
            let mut word_start = None;
            for (i, c) in line
                .char_indices()
                .chain(std::iter::once((line.len(), ' ')))
            {
                let in_word = c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.');
                match (word_start, in_word) {
                    (None, true) => word_start = Some(i),
                    (Some(start), false) => {
                        if let Some(kind) = detect(&line[start..i]) {
                            findings.push(Finding {
                                kind,
                                start: offset + start,
                                end: offset + i,
                            });
                        }
                        word_start = None;
                    }
                    _ => {}
                }
            }
        }
        offset += line.len();
    }
    findings
}

#[cfg(test)]
#[path = "scan.test.rs"]
mod tests;

#[cfg(test)]
#[path = "scan.spec.rs"]
mod spec;
