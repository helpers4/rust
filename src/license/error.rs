// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string is not a valid SPDX license expression.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseExpressionError {
    /// The expression is empty or only whitespace.
    Empty,
    /// The expression stops where a license or an operand is expected (`MIT OR`).
    UnexpectedEnd,
    /// A token that cannot come here: two licenses in a row, a stray `)`, an operator with
    /// nothing before it.
    UnexpectedToken {
        /// Byte offset of the token in the expression.
        index: usize,
    },
    /// A license identifier with a character outside letters, digits, `.`, `-` and `:`.
    InvalidIdentifier {
        /// Byte offset of the identifier in the expression.
        index: usize,
    },
    /// An opening parenthesis that is never closed.
    UnclosedParenthesis {
        /// Byte offset of the `(`.
        index: usize,
    },
}

impl fmt::Display for ParseExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty license expression"),
            Self::UnexpectedEnd => write!(f, "the expression ends too early"),
            Self::UnexpectedToken { index } => write!(f, "unexpected token at byte {index}"),
            Self::InvalidIdentifier { index } => {
                write!(f, "invalid license identifier at byte {index}")
            }
            Self::UnclosedParenthesis { index } => {
                write!(f, "unclosed parenthesis at byte {index}")
            }
        }
    }
}

impl std::error::Error for ParseExpressionError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
