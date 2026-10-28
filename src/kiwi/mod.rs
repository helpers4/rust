// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Decoding of the [Kiwi](https://github.com/evanw/kiwi) binary format, Evan Wallace's
//! schema-based serialization (used by Figma's `.fig` files, but generic).
//!
//! A Kiwi message is meaningless without its schema. [`Schema::parse`] reads a *binary* schema
//! (the compiled form Kiwi writes, which `.fig` files embed), then [`Schema::decode`] decodes a
//! message against one of its definitions into a dynamic [`Value`] tree: objects, arrays, enums
//! by name and the usual scalars. Every failure on truncated or invalid data is a precise
//! [`KiwiError`].
//!
//! Built on [`bytes::ByteReader`](crate::bytes::ByteReader), plus Kiwi's own 64-bit varints and
//! floats, which are not LEB128 and IEEE 754 on the wire.

mod _reader;
#[cfg(test)]
mod _test_support;
mod definition;
mod error;
mod schema;
mod value;

pub use definition::Definition;
pub use definition::Field;
pub use definition::FieldType;
pub use definition::Kind;
pub use error::KiwiError;
pub use schema::Schema;
pub use value::Value;
