// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a Kiwi schema or message could not be decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum KiwiError {
    /// The data ends in the middle of a value.
    UnexpectedEof {
        /// Byte offset where the read that fell short started.
        index: usize,
    },
    /// A string that is not valid UTF-8, or has no terminating `0`.
    InvalidString {
        /// Byte offset where the string starts.
        index: usize,
    },
    /// A definition whose kind byte is not enum, struct or message.
    UnknownKind {
        /// The kind byte.
        kind: u8,
    },
    /// A field whose type is neither a built-in type nor a definition of the schema.
    UnknownType {
        /// The raw type number.
        type_id: i32,
    },
    /// No definition has the requested name.
    UnknownDefinition {
        /// The name that was asked for.
        name: String,
    },
    /// A message holds a field id its definition does not list.
    UnknownField {
        /// The message definition.
        message: String,
        /// The field id.
        id: u32,
    },
    /// The data nests deeper than the decoder allows (a malformed or hostile schema can recurse
    /// forever).
    TooDeep,
}

impl fmt::Display for KiwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof { index } => write!(f, "unexpected end of data at byte {index}"),
            Self::InvalidString { index } => write!(f, "invalid string at byte {index}"),
            Self::UnknownKind { kind } => write!(f, "unknown definition kind {kind}"),
            Self::UnknownType { type_id } => write!(f, "unknown type {type_id}"),
            Self::UnknownDefinition { name } => write!(f, "no definition named {name:?}"),
            Self::UnknownField { message, id } => {
                write!(f, "field id {id} does not exist in message {message:?}")
            }
            Self::TooDeep => write!(f, "data nested too deeply"),
        }
    }
}

impl std::error::Error for KiwiError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
