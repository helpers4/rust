// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::sync::Arc;

/// What a [`Definition`] is: an enum, a struct (all its fields, in order) or a message (any of
/// its fields, each with an id, ended by a `0`).
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::Kind;
///
/// assert_ne!(Kind::Enum, Kind::Message);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    /// A set of named numbers.
    Enum,
    /// A fixed list of fields, always all present.
    Struct,
    /// A list of optional fields, each written with its id.
    Message,
}

/// The type of a [`Field`]: one of Kiwi's built-in types, or another definition of the schema.
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::FieldType;
///
/// assert_eq!(FieldType::Definition(2), FieldType::Definition(2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FieldType {
    /// One byte, `0` is false.
    Bool,
    /// An unsigned byte.
    Byte,
    /// A signed 32-bit integer.
    Int,
    /// An unsigned 32-bit integer.
    Uint,
    /// A 32-bit float.
    Float,
    /// A UTF-8 string.
    String,
    /// A signed 64-bit integer.
    Int64,
    /// An unsigned 64-bit integer.
    Uint64,
    /// Another definition, by its index in [`Schema::definitions`](super::Schema::definitions).
    Definition(usize),
}

/// One field of a [`Definition`]. On an enum, a field is a member: only its name and `value`
/// matter.
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::{Field, FieldType};
///
/// let field = Field { name: "id".into(), field_type: Some(FieldType::Uint), is_array: false, value: 1 };
/// assert_eq!(&*field.name, "id");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The field name.
    pub name: Arc<str>,
    /// The type, `None` on an enum member.
    pub field_type: Option<FieldType>,
    /// Whether the field holds a list of values.
    pub is_array: bool,
    /// The field id of a message field, or the number of an enum member.
    pub value: u32,
}

/// A named enum, struct or message of a Kiwi schema.
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::{Definition, Kind};
///
/// let empty = Definition { name: "Empty".into(), kind: Kind::Message, fields: vec![] };
/// assert_eq!(&*empty.name, "Empty");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// The definition name.
    pub name: Arc<str>,
    /// Whether it is an enum, a struct or a message.
    pub kind: Kind,
    /// Its fields (or enum members), in schema order.
    pub fields: Vec<Field>,
}
