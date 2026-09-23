// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{ParseExpressionError, lookup};
use std::fmt;
use std::str::FromStr;

/// An [SPDX license expression](https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/):
/// one license, or several combined with `AND` and `OR`.
///
/// The syntax is `MIT`, `GPL-2.0-or-later`, `Apache-2.0 WITH LLVM-exception`, `MIT OR Apache-2.0`
/// and `(MIT OR Apache-2.0) AND BSD-3-Clause`, with `WITH` binding tighter than `AND`, and `AND`
/// tighter than `OR`. Operators are upper case, and the legacy `MIT/Apache-2.0` of older Cargo
/// manifests is read as `OR`. A trailing `+` on an identifier is kept (`GPL-2.0+`). Identifiers
/// are only checked for their shape: use [`unknown_ids`](Self::unknown_ids) to find the ones this
/// crate does not list. `Display` writes the expression back, with parentheses only where they
/// change the meaning.
///
/// # Examples
///
/// ```
/// use helpers4::license::Expression;
///
/// let expr = Expression::parse("(MIT OR Apache-2.0) AND BSD-3-Clause")?;
/// assert_eq!(expr.licenses(), ["MIT", "Apache-2.0", "BSD-3-Clause"]);
/// // A policy that allows MIT and BSD-3-Clause is satisfied:
/// assert!(expr.is_satisfied_by(&["MIT", "BSD-3-Clause"]));
/// // ... one that only allows MIT is not.
/// assert!(!expr.is_satisfied_by(&["MIT"]));
/// # Ok::<(), helpers4::license::ParseExpressionError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    root: Node,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    License {
        id: String,
        exception: Option<String>,
    },
    /// Two or more operands that must all be satisfied (nested `AND`s are flattened).
    And(Vec<Node>),
    /// Two or more operands of which one must be satisfied (nested `OR`s are flattened).
    Or(Vec<Node>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    Open,
    Close,
    Slash,
    Word(&'a str),
}

impl Expression {
    /// Parses an SPDX license expression.
    ///
    /// # Arguments
    ///
    /// - `text` - The expression, such as `"MIT OR Apache-2.0"`.
    ///
    /// # Errors
    ///
    /// A [`ParseExpressionError`] for an empty expression, one that ends too early, a token in
    /// the wrong place, a malformed identifier, or an unclosed parenthesis.
    pub fn parse(text: &str) -> Result<Self, ParseExpressionError> {
        let tokens = tokenize(text);
        if tokens.is_empty() {
            return Err(ParseExpressionError::Empty);
        }
        let mut parser = Parser {
            tokens: &tokens,
            position: 0,
        };
        let root = parser.or()?;
        match parser.tokens.get(parser.position) {
            None => Ok(Self { root }),
            Some(&(index, _)) => Err(ParseExpressionError::UnexpectedToken { index }),
        }
    }

    /// The license identifiers used, in order of first appearance, without repeats.
    ///
    /// # Returns
    ///
    /// The identifiers as written, without the exceptions after `WITH`.
    #[must_use]
    pub fn licenses(&self) -> Vec<&str> {
        let mut found = Vec::new();
        self.root.collect(&mut found);
        found
    }

    /// The identifiers that are neither in this crate's list of licenses (see
    /// [`lookup`](super::lookup)) nor a `LicenseRef-` / `DocumentRef-` reference.
    ///
    /// # Returns
    ///
    /// The identifiers to double-check: a typo, or a license this crate does not list.
    #[must_use]
    pub fn unknown_ids(&self) -> Vec<&str> {
        self.licenses()
            .into_iter()
            .filter(|id| {
                let base = id.strip_suffix('+').unwrap_or(id);
                !base.starts_with("LicenseRef-")
                    && !base.starts_with("DocumentRef-")
                    && lookup(base).is_none()
            })
            .collect()
    }

    /// Whether a policy that allows exactly the licenses in `allowed` accepts this expression:
    /// `OR` needs one side, `AND` needs both, and a `WITH` exception never makes a license less
    /// acceptable than the license alone.
    ///
    /// Identifiers are compared ignoring case and exactly otherwise (`GPL-2.0+` is not
    /// `GPL-2.0-or-later`; call [`normalize`](super::normalize) first if that matters).
    ///
    /// # Arguments
    ///
    /// - `allowed` - The identifiers the policy accepts.
    ///
    /// # Returns
    ///
    /// `true` when the expression can be satisfied with those licenses.
    #[must_use]
    pub fn is_satisfied_by(&self, allowed: &[&str]) -> bool {
        self.root.satisfied_by(allowed)
    }
}

impl Node {
    /// One operand is itself; several become `make(...)`, with nested nodes of the same kind
    /// flattened so that `A AND (B AND C)` and `A AND B AND C` are the same expression.
    fn combine(mut operands: Vec<Self>, make: fn(Vec<Self>) -> Self) -> Self {
        if operands.len() == 1 {
            return operands.swap_remove(0);
        }
        let kind = make(Vec::new());
        let mut flat = Vec::new();
        for operand in operands {
            match (&kind, operand) {
                (Self::And(_), Self::And(inner)) | (Self::Or(_), Self::Or(inner)) => {
                    flat.extend(inner);
                }
                (_, other) => flat.push(other),
            }
        }
        make(flat)
    }

    fn collect<'a>(&'a self, found: &mut Vec<&'a str>) {
        match self {
            Self::License { id, .. } => {
                if !found.contains(&id.as_str()) {
                    found.push(id);
                }
            }
            Self::And(operands) | Self::Or(operands) => {
                for operand in operands {
                    operand.collect(found);
                }
            }
        }
    }

    fn satisfied_by(&self, allowed: &[&str]) -> bool {
        match self {
            Self::License { id, .. } => allowed.iter().any(|a| a.eq_ignore_ascii_case(id)),
            Self::And(operands) => operands.iter().all(|operand| operand.satisfied_by(allowed)),
            Self::Or(operands) => operands.iter().any(|operand| operand.satisfied_by(allowed)),
        }
    }

    /// Writes the node; `parent_is_and` says whether an `OR` here needs parentheses.
    fn write(&self, out: &mut String, parent_is_and: bool) {
        match self {
            Self::License { id, exception } => {
                out.push_str(id);
                if let Some(exception) = exception {
                    out.push_str(" WITH ");
                    out.push_str(exception);
                }
            }
            Self::And(operands) => {
                for (i, operand) in operands.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" AND ");
                    }
                    operand.write(out, true);
                }
            }
            Self::Or(operands) => {
                if parent_is_and {
                    out.push('(');
                }
                for (i, operand) in operands.iter().enumerate() {
                    if i > 0 {
                        out.push_str(" OR ");
                    }
                    operand.write(out, false);
                }
                if parent_is_and {
                    out.push(')');
                }
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.root.write(&mut out, false);
        f.write_str(&out)
    }
}

impl FromStr for Expression {
    type Err = ParseExpressionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

/// Cuts the text into parentheses, `/` and words, with the byte offset of each.
fn tokenize(text: &str) -> Vec<(usize, Token<'_>)> {
    let mut tokens = Vec::new();
    let mut word_start = None;
    for (i, c) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        let special = matches!(c, '(' | ')' | '/') || c.is_whitespace();
        match (word_start, special) {
            (None, false) => word_start = Some(i),
            (Some(start), true) => {
                tokens.push((start, Token::Word(&text[start..i])));
                word_start = None;
            }
            _ => {}
        }
        if i < text.len() {
            match c {
                '(' => tokens.push((i, Token::Open)),
                ')' => tokens.push((i, Token::Close)),
                '/' => tokens.push((i, Token::Slash)),
                _ => {}
            }
        }
    }
    tokens
}

struct Parser<'a> {
    tokens: &'a [(usize, Token<'a>)],
    position: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<(usize, Token<'a>)> {
        self.tokens.get(self.position).copied()
    }

    fn is_keyword(&self, keyword: &str) -> bool {
        matches!(self.peek(), Some((_, Token::Word(word))) if word == keyword)
    }

    fn or(&mut self) -> Result<Node, ParseExpressionError> {
        let mut operands = vec![self.and()?];
        while self.is_keyword("OR") || matches!(self.peek(), Some((_, Token::Slash))) {
            self.position += 1;
            operands.push(self.and()?);
        }
        Ok(Node::combine(operands, Node::Or))
    }

    fn and(&mut self) -> Result<Node, ParseExpressionError> {
        let mut operands = vec![self.simple()?];
        while self.is_keyword("AND") {
            self.position += 1;
            operands.push(self.simple()?);
        }
        Ok(Node::combine(operands, Node::And))
    }

    fn simple(&mut self) -> Result<Node, ParseExpressionError> {
        let (index, token) = self.peek().ok_or(ParseExpressionError::UnexpectedEnd)?;
        match token {
            Token::Open => {
                self.position += 1;
                let inner = self.or()?;
                match self.peek() {
                    Some((_, Token::Close)) => {
                        self.position += 1;
                        Ok(inner)
                    }
                    _ => Err(ParseExpressionError::UnclosedParenthesis { index }),
                }
            }
            Token::Word(word) if !matches!(word, "AND" | "OR" | "WITH") => {
                if !is_identifier(word) {
                    return Err(ParseExpressionError::InvalidIdentifier { index });
                }
                self.position += 1;
                let exception = if self.is_keyword("WITH") {
                    self.position += 1;
                    let (index, token) = self.peek().ok_or(ParseExpressionError::UnexpectedEnd)?;
                    match token {
                        Token::Word(name)
                            if is_identifier(name) && !matches!(name, "AND" | "OR" | "WITH") =>
                        {
                            self.position += 1;
                            Some(name.to_string())
                        }
                        _ => return Err(ParseExpressionError::UnexpectedToken { index }),
                    }
                } else {
                    None
                };
                Ok(Node::License {
                    id: word.to_string(),
                    exception,
                })
            }
            _ => Err(ParseExpressionError::UnexpectedToken { index }),
        }
    }
}

/// An identifier: letters, digits, `.`, `-` and `:`, with an optional trailing `+`.
fn is_identifier(word: &str) -> bool {
    let body = word.strip_suffix('+').unwrap_or(word);
    !body.is_empty()
        && body
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b':'))
}

#[cfg(test)]
#[path = "expression.test.rs"]
mod tests;

#[cfg(test)]
#[path = "expression.spec.rs"]
mod spec;
