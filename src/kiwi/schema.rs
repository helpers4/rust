// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_reader::Reader;
use super::{Definition, Field, FieldType, Kind, KiwiError, Value};
use std::sync::Arc;

/// How deep nested values may go before decoding stops with [`KiwiError::TooDeep`].
const MAX_DEPTH: usize = 64;

/// A Kiwi schema: the definitions (enums, structs and messages) that give meaning to the bytes
/// of a message.
///
/// Read one from its binary form with [`Schema::parse`], then decode messages with
/// [`Schema::decode`].
///
/// # Examples
///
/// ```
/// use helpers4::kiwi::Schema;
///
/// // A schema with one message, `Ping`, holding a `uint` field `id` (field number 1).
/// let schema = Schema::parse(b"\x01Ping\0\x02\x01id\0\x07\x00\x01")?;
/// let ping = schema.decode(&[1, 42, 0], "Ping")?;
/// assert_eq!(ping.get("id").and_then(|id| id.as_u32()), Some(42));
/// # Ok::<(), helpers4::kiwi::KiwiError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    /// The definitions, in schema order: [`FieldType::Definition`] refers to one by index.
    pub definitions: Vec<Definition>,
}

impl Schema {
    /// Reads a binary Kiwi schema (what Kiwi's `compileSchema` / `encodeBinarySchema` write; in
    /// a `.fig` file, the first decompressed chunk).
    ///
    /// # Arguments
    ///
    /// - `bytes` - The binary schema. Bytes after the last definition are ignored.
    ///
    /// # Returns
    ///
    /// The schema.
    ///
    /// # Errors
    ///
    /// A [`KiwiError`] on truncated data, an invalid string, an unknown definition kind, or a
    /// field whose type is not a built-in type nor a definition of the schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::kiwi::{Kind, Schema};
    ///
    /// let schema = Schema::parse(b"\x01Ping\0\x02\x01id\0\x07\x00\x01")?;
    /// assert_eq!(&*schema.definitions[0].name, "Ping");
    /// assert_eq!(schema.definitions[0].kind, Kind::Message);
    /// # Ok::<(), helpers4::kiwi::KiwiError>(())
    /// ```
    pub fn parse(bytes: &[u8]) -> Result<Self, KiwiError> {
        let mut reader = Reader::new(bytes);
        let count = reader.var_uint()? as usize;
        // Every definition takes at least three bytes: do not trust a huge count.
        let mut raw = Vec::with_capacity(count.min(reader.remaining()));
        for _ in 0..count {
            raw.push(read_definition(&mut reader)?);
        }
        let known = raw.len();
        let definitions = raw
            .into_iter()
            .map(|(name, kind, fields)| {
                let fields = fields
                    .into_iter()
                    .map(|(name, raw_type, is_array, value)| {
                        let field_type = raw_type
                            .map(|type_id| resolve_type(type_id, known))
                            .transpose()?;
                        Ok(Field {
                            name,
                            field_type,
                            is_array,
                            value,
                        })
                    })
                    .collect::<Result<Vec<_>, KiwiError>>()?;
                Ok(Definition { name, kind, fields })
            })
            .collect::<Result<Vec<_>, KiwiError>>()?;
        Ok(Self { definitions })
    }

    /// The index of the definition with this name.
    ///
    /// # Arguments
    ///
    /// - `name` - The definition name.
    ///
    /// # Returns
    ///
    /// The index in [`definitions`](Self::definitions), or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::kiwi::Schema;
    ///
    /// let schema = Schema::parse(b"\x01Ping\0\x02\x00")?;
    /// assert_eq!(schema.find("Ping"), Some(0));
    /// assert_eq!(schema.find("Pong"), None);
    /// # Ok::<(), helpers4::kiwi::KiwiError>(())
    /// ```
    #[must_use]
    pub fn find(&self, name: &str) -> Option<usize> {
        self.definitions.iter().position(|d| &*d.name == name)
    }

    /// Decodes a message as an instance of one definition (usually a message, the root of the
    /// file). Bytes after the value are left alone.
    ///
    /// # Arguments
    ///
    /// - `bytes` - The encoded value.
    /// - `root` - The name of the definition it is an instance of.
    ///
    /// # Returns
    ///
    /// The value: an [`Value::Object`] for a message or a struct, with enums by name and a
    /// `byte[]` as [`Value::Bytes`].
    ///
    /// # Errors
    ///
    /// A [`KiwiError`] when `root` is unknown, the data is truncated or invalid (a message with a
    /// field id its definition lacks, a bad string), or it nests deeper than 64 levels.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::kiwi::{KiwiError, Schema};
    ///
    /// let schema = Schema::parse(b"\x01Ping\0\x02\x01id\0\x07\x00\x01")?;
    /// assert_eq!(schema.decode(&[1], "Ping"), Err(KiwiError::UnexpectedEof { index: 1 }));
    /// assert!(schema.decode(&[0], "Pong").is_err());
    /// # Ok::<(), KiwiError>(())
    /// ```
    pub fn decode(&self, bytes: &[u8], root: &str) -> Result<Value, KiwiError> {
        let index = self
            .find(root)
            .ok_or_else(|| KiwiError::UnknownDefinition {
                name: root.to_string(),
            })?;
        self.decode_definition(index, &mut Reader::new(bytes), 0)
    }

    fn decode_definition(
        &self,
        index: usize,
        reader: &mut Reader<'_>,
        depth: usize,
    ) -> Result<Value, KiwiError> {
        if depth > MAX_DEPTH {
            return Err(KiwiError::TooDeep);
        }
        let Some(definition) = self.definitions.get(index) else {
            return Err(KiwiError::UnknownType {
                type_id: i32::try_from(index).unwrap_or(i32::MAX),
            });
        };
        match definition.kind {
            Kind::Enum => {
                let raw = reader.var_uint()?;
                Ok(definition
                    .fields
                    .iter()
                    .find(|member| member.value == raw)
                    .map_or(Value::UnknownEnum(raw), |member| {
                        Value::Enum(member.name.clone())
                    }))
            }
            Kind::Struct => {
                let mut fields = Vec::with_capacity(definition.fields.len());
                for field in &definition.fields {
                    fields.push((field.name.clone(), self.decode_field(field, reader, depth)?));
                }
                Ok(Value::Object(fields))
            }
            Kind::Message => {
                let mut fields = Vec::new();
                loop {
                    let id = reader.var_uint()?;
                    if id == 0 {
                        return Ok(Value::Object(fields));
                    }
                    let field = definition
                        .fields
                        .iter()
                        .find(|field| field.value == id)
                        .ok_or_else(|| KiwiError::UnknownField {
                            message: definition.name.to_string(),
                            id,
                        })?;
                    fields.push((field.name.clone(), self.decode_field(field, reader, depth)?));
                }
            }
        }
    }

    fn decode_field(
        &self,
        field: &Field,
        reader: &mut Reader<'_>,
        depth: usize,
    ) -> Result<Value, KiwiError> {
        // Enum members have no type; a field built by hand without one reads as a `uint`.
        let field_type = field.field_type.unwrap_or(FieldType::Uint);
        if !field.is_array {
            return self.decode_value(field_type, reader, depth);
        }
        let length = reader.var_uint()? as usize;
        if field_type == FieldType::Byte {
            return Ok(Value::Bytes(reader.take(length)?.to_vec()));
        }
        // Every item takes at least one byte: do not trust a huge length.
        let mut items = Vec::with_capacity(length.min(reader.remaining()));
        for _ in 0..length {
            items.push(self.decode_value(field_type, reader, depth)?);
        }
        Ok(Value::Array(items))
    }

    fn decode_value(
        &self,
        field_type: FieldType,
        reader: &mut Reader<'_>,
        depth: usize,
    ) -> Result<Value, KiwiError> {
        Ok(match field_type {
            FieldType::Bool => Value::Bool(reader.byte()? != 0),
            FieldType::Byte => Value::Byte(reader.byte()?),
            FieldType::Int => Value::Int(reader.var_int()?),
            FieldType::Uint => Value::Uint(reader.var_uint()?),
            FieldType::Float => Value::Float(reader.var_float()?),
            FieldType::String => Value::String(reader.string()?),
            FieldType::Int64 => Value::Int64(reader.var_int64()?),
            FieldType::Uint64 => Value::Uint64(reader.var_uint64()?),
            FieldType::Definition(index) => self.decode_definition(index, reader, depth + 1)?,
        })
    }
}

type RawField = (Arc<str>, Option<i32>, bool, u32);

fn read_definition(reader: &mut Reader<'_>) -> Result<(Arc<str>, Kind, Vec<RawField>), KiwiError> {
    let name: Arc<str> = reader.string()?.into();
    let kind = match reader.byte()? {
        0 => Kind::Enum,
        1 => Kind::Struct,
        2 => Kind::Message,
        kind => return Err(KiwiError::UnknownKind { kind }),
    };
    let count = reader.var_uint()? as usize;
    // Every field takes at least four bytes: do not trust a huge count.
    let mut fields = Vec::with_capacity(count.min(reader.remaining()));
    for _ in 0..count {
        let field_name: Arc<str> = reader.string()?.into();
        let raw_type = reader.var_int()?;
        let is_array = reader.byte()? & 1 == 1;
        let value = reader.var_uint()?;
        // An enum member carries a type number too, but it means nothing.
        fields.push((
            field_name,
            (kind != Kind::Enum).then_some(raw_type),
            is_array,
            value,
        ));
    }
    Ok((name, kind, fields))
}

fn resolve_type(type_id: i32, definition_count: usize) -> Result<FieldType, KiwiError> {
    Ok(match type_id {
        -1 => FieldType::Bool,
        -2 => FieldType::Byte,
        -3 => FieldType::Int,
        -4 => FieldType::Uint,
        -5 => FieldType::Float,
        -6 => FieldType::String,
        -7 => FieldType::Int64,
        -8 => FieldType::Uint64,
        _ => match usize::try_from(type_id) {
            Ok(index) if index < definition_count => FieldType::Definition(index),
            _ => return Err(KiwiError::UnknownType { type_id }),
        },
    })
}

#[cfg(test)]
#[path = "schema.test.rs"]
mod tests;

#[cfg(test)]
#[path = "schema.spec.rs"]
mod spec;
