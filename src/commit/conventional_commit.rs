// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Bump, ParseCommitError};
use std::fmt;
use std::str::FromStr;

/// A parsed [Conventional Commits 1.0.0](https://www.conventionalcommits.org) message:
/// `type(scope)!: description`, then an optional body and optional footers.
///
/// The parser follows the specification: the header is `type`, an optional `(scope)`, an optional
/// `!` and `: `, then the description; a blank line separates the header from the body; footers
/// (`Token: value` or `Token #value`, with the token in words joined by `-` or exactly
/// `BREAKING CHANGE`) start at the first such line that follows a blank line and run to the end.
/// A commit is breaking when it has the `!` or a `BREAKING CHANGE` footer. Text such as an
/// emoji before the description is part of the description.
///
/// # Examples
///
/// ```
/// use helpers4::commit::{Bump, Commit};
///
/// let commit = Commit::parse("feat(api)!: drop the v1 routes\n\nThey were deprecated.\n\nRefs #42")?;
/// assert_eq!(commit.kind(), "feat");
/// assert_eq!(commit.scope(), Some("api"));
/// assert!(commit.is_breaking());
/// assert_eq!(commit.description(), "drop the v1 routes");
/// assert_eq!(commit.body(), Some("They were deprecated."));
/// assert_eq!(commit.footer("refs"), Some("42"));
/// assert_eq!(commit.bump(), Bump::Major);
/// # Ok::<(), helpers4::commit::ParseCommitError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    kind: String,
    scope: Option<String>,
    breaking: bool,
    description: String,
    body: Option<String>,
    footers: Vec<(String, String)>,
}

impl Commit {
    /// Parses a commit message.
    ///
    /// # Arguments
    ///
    /// - `message` - The whole message: header, then optionally a blank line, a body and footers.
    ///
    /// # Errors
    ///
    /// A [`ParseCommitError`] when the message is empty, the header does not look like
    /// `type(scope): description`, or the line after the header is not blank.
    pub fn parse(message: &str) -> Result<Self, ParseCommitError> {
        let message = message.trim_end();
        let mut lines = message.lines();
        let header = lines.next().unwrap_or("");
        if header.trim().is_empty() {
            return Err(ParseCommitError::Empty);
        }
        let rest: Vec<&str> = lines.collect();
        let header = parse_header(header)?;
        if rest.first().is_some_and(|line| !line.trim().is_empty()) {
            return Err(ParseCommitError::MissingBlankLine);
        }
        let content: Vec<&str> = rest
            .iter()
            .skip_while(|line| line.trim().is_empty())
            .copied()
            .collect();
        let footer_start = (0..content.len())
            .find(|&i| {
                (i == 0 || content[i - 1].trim().is_empty()) && split_footer(content[i]).is_some()
            })
            .unwrap_or(content.len());
        let body = content[..footer_start].join("\n").trim_end().to_string();
        let footers = parse_footers(&content[footer_start..]);
        let breaking = header.breaking || footers.iter().any(|(token, _)| is_breaking_token(token));
        Ok(Self {
            kind: header.kind,
            scope: header.scope,
            breaking,
            description: header.description,
            body: if body.is_empty() { None } else { Some(body) },
            footers,
        })
    }

    /// The commit type as written, such as `"feat"` or `"fix"`.
    ///
    /// # Returns
    ///
    /// The type, without the scope or the `!`.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// The scope between the parentheses.
    ///
    /// # Returns
    ///
    /// The scope, or `None` when there is none.
    #[must_use]
    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    /// Whether the commit is a breaking change: `!` in the header or a `BREAKING CHANGE` footer.
    ///
    /// # Returns
    ///
    /// `true` for a breaking change.
    #[must_use]
    pub fn is_breaking(&self) -> bool {
        self.breaking
    }

    /// The description after the `: `, trimmed.
    ///
    /// # Returns
    ///
    /// The one-line summary.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// The free-form body between the header and the footers.
    ///
    /// # Returns
    ///
    /// The body with its line breaks, or `None` when there is none.
    #[must_use]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    /// Every footer, in order, as `(token, value)`.
    ///
    /// # Returns
    ///
    /// The footers, such as `("Refs", "42")` or `("BREAKING CHANGE", "the API changed")`.
    #[must_use]
    pub fn footers(&self) -> &[(String, String)] {
        &self.footers
    }

    /// The value of the first footer with the given token, ignoring case.
    ///
    /// # Arguments
    ///
    /// - `token` - The footer token, such as `"Refs"` or `"BREAKING CHANGE"`.
    ///
    /// # Returns
    ///
    /// The value, or `None` when there is no such footer.
    #[must_use]
    pub fn footer(&self, token: &str) -> Option<&str> {
        self.footers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(token))
            .map(|(_, value)| value.as_str())
    }

    /// The version bump this commit calls for under Semantic Versioning: [`Bump::Major`] for a
    /// breaking change, [`Bump::Minor`] for `feat`, [`Bump::Patch`] for `fix`, otherwise
    /// [`Bump::None`]. The type is matched ignoring case.
    ///
    /// # Returns
    ///
    /// The bump level.
    #[must_use]
    pub fn bump(&self) -> Bump {
        if self.breaking {
            Bump::Major
        } else if self.kind.eq_ignore_ascii_case("feat") {
            Bump::Minor
        } else if self.kind.eq_ignore_ascii_case("fix") {
            Bump::Patch
        } else {
            Bump::None
        }
    }
}

impl fmt::Display for Commit {
    /// Writes the message back: `type(scope)!: description`, the body, then the footers as
    /// `token: value`. Parsing the result gives the same commit.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = self.kind.clone();
        if let Some(scope) = &self.scope {
            out.push('(');
            out.push_str(scope);
            out.push(')');
        }
        let has_bang_footer = self
            .footers
            .iter()
            .any(|(token, _)| is_breaking_token(token));
        if self.breaking && !has_bang_footer {
            out.push('!');
        }
        out.push_str(": ");
        out.push_str(&self.description);
        if let Some(body) = &self.body {
            out.push_str("\n\n");
            out.push_str(body);
        }
        if !self.footers.is_empty() {
            out.push('\n');
        }
        for (token, value) in &self.footers {
            out.push('\n');
            out.push_str(token);
            out.push_str(": ");
            out.push_str(value);
        }
        f.write_str(&out)
    }
}

impl FromStr for Commit {
    type Err = ParseCommitError;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        Self::parse(message)
    }
}

struct Header {
    kind: String,
    scope: Option<String>,
    breaking: bool,
    description: String,
}

fn parse_header(header: &str) -> Result<Header, ParseCommitError> {
    let colon = header.find(':').ok_or(ParseCommitError::MissingSeparator)?;
    let (prefix, after) = (&header[..colon], &header[colon + 1..]);
    let description = match after.strip_prefix(' ') {
        Some(description) => description.trim(),
        None if after.is_empty() => "",
        None => return Err(ParseCommitError::MissingSeparator),
    };
    if description.is_empty() {
        return Err(ParseCommitError::EmptyDescription);
    }
    let (prefix, breaking) = match prefix.strip_suffix('!') {
        Some(prefix) => (prefix, true),
        None => (prefix, false),
    };
    let (kind, scope) = match prefix.split_once('(') {
        Some((kind, rest)) => {
            let scope = rest
                .strip_suffix(')')
                .ok_or(ParseCommitError::InvalidScope)?;
            if scope.is_empty() || scope.contains(['(', ')']) {
                return Err(ParseCommitError::InvalidScope);
            }
            (kind, Some(scope.to_string()))
        }
        None => (prefix, None),
    };
    if kind.is_empty()
        || !kind
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(ParseCommitError::InvalidType);
    }
    Ok(Header {
        kind: kind.to_string(),
        scope,
        breaking,
        description: description.to_string(),
    })
}

/// Splits a footer line into `(token, value)`: `Token: value`, `Token #value` or
/// `BREAKING CHANGE: value`.
fn split_footer(line: &str) -> Option<(&str, &str)> {
    for token in ["BREAKING CHANGE", "BREAKING-CHANGE"] {
        if let Some(value) = line
            .strip_prefix(token)
            .and_then(|rest| rest.strip_prefix(": "))
        {
            return Some((token, value));
        }
    }
    let end = line.find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))?;
    let (token, rest) = line.split_at(end);
    if token.is_empty() {
        return None;
    }
    rest.strip_prefix(": ")
        .or_else(|| rest.strip_prefix(" #"))
        .map(|value| (token, value))
}

fn parse_footers(lines: &[&str]) -> Vec<(String, String)> {
    let starts: Vec<usize> = (0..lines.len())
        .filter(|&i| split_footer(lines[i]).is_some())
        .collect();
    starts
        .iter()
        .enumerate()
        .filter_map(|(n, &start)| {
            let end = starts.get(n + 1).copied().unwrap_or(lines.len());
            split_footer(lines[start]).map(|(token, first)| {
                let mut value = first.to_string();
                for line in &lines[start + 1..end] {
                    value.push('\n');
                    value.push_str(line);
                }
                (token.to_string(), value.trim_end().to_string())
            })
        })
        .collect()
}

fn is_breaking_token(token: &str) -> bool {
    token == "BREAKING CHANGE" || token == "BREAKING-CHANGE"
}

#[cfg(test)]
#[path = "conventional_commit.test.rs"]
mod tests;

#[cfg(test)]
#[path = "conventional_commit.spec.rs"]
mod spec;
