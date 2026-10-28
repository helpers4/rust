// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::sync::Arc;

/// A decoded Kiwi value. Messages and structs are [`Object`](Self::Object)s that keep the field
/// order of the data.
///
/// The `as_*` accessors return `None` when the value is of another type, so a chain of `get`
/// and `as_*` reads like a path.
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::Value;
///
/// let node = Value::Object(vec![
///     ("name".into(), Value::String("Frame 1".to_string())),
///     ("size".into(), Value::Float(10.0)),
/// ]);
/// assert_eq!(node.get("name").and_then(Value::as_str), Some("Frame 1"));
/// assert_eq!(node.get("size").and_then(Value::as_f32), Some(10.0));
/// assert_eq!(node.get("missing"), None);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    /// A `bool`.
    Bool(bool),
    /// A `byte`.
    Byte(u8),
    /// An `int`.
    Int(i32),
    /// A `uint`.
    Uint(u32),
    /// A `float`.
    Float(f32),
    /// A `string`.
    String(String),
    /// An `int64`.
    Int64(i64),
    /// A `uint64`.
    Uint64(u64),
    /// A `byte[]`, kept as bytes.
    Bytes(Vec<u8>),
    /// An enum member, by name.
    Enum(Arc<str>),
    /// An enum number the schema does not list.
    UnknownEnum(u32),
    /// A list of values.
    Array(Vec<Value>),
    /// A struct or message: its fields by name, in data order.
    Object(Vec<(Arc<str>, Value)>),
}

impl Value {
    /// A field of an object.
    ///
    /// # Arguments
    ///
    /// - `name` - The field name.
    ///
    /// # Returns
    ///
    /// The field, or `None` when this is not an object or has no such field (a message omits its
    /// absent fields).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Self> {
        match self {
            Self::Object(fields) => fields.iter().find(|(n, _)| &**n == name).map(|(_, v)| v),
            _ => None,
        }
    }

    /// The text of a string or of an enum member name.
    ///
    /// # Returns
    ///
    /// The text, or `None` for any other value.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text),
            Self::Enum(name) => Some(name),
            _ => None,
        }
    }

    /// The value of a `bool`.
    ///
    /// # Returns
    ///
    /// The boolean, or `None` for any other value.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// The value of an `int`.
    ///
    /// # Returns
    ///
    /// The integer, or `None` for any other value.
    #[must_use]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// The value of a `uint`.
    ///
    /// # Returns
    ///
    /// The integer, or `None` for any other value.
    #[must_use]
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::Uint(value) => Some(*value),
            _ => None,
        }
    }

    /// The value of an `int64`.
    ///
    /// # Returns
    ///
    /// The integer, or `None` for any other value.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int64(value) => Some(*value),
            _ => None,
        }
    }

    /// The value of a `uint64`.
    ///
    /// # Returns
    ///
    /// The integer, or `None` for any other value.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Uint64(value) => Some(*value),
            _ => None,
        }
    }

    /// The value of a `float`.
    ///
    /// # Returns
    ///
    /// The float, or `None` for any other value.
    #[must_use]
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// The bytes of a `byte[]`.
    ///
    /// # Returns
    ///
    /// The bytes, or `None` for any other value.
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// The items of an array.
    ///
    /// # Returns
    ///
    /// The items, or an empty slice for any other value, so a missing list reads as an empty one.
    #[must_use]
    pub fn as_array(&self) -> &[Self] {
        match self {
            Self::Array(items) => items,
            _ => &[],
        }
    }
}

#[cfg(test)]
#[path = "value.test.rs"]
mod tests;
