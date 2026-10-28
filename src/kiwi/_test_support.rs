// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Encoders for the unit tests, shared by `schema.test.rs` and `schema.spec.rs`.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub(super) fn var_uint(mut value: u64, out: &mut Vec<u8>) {
    while value >= 0x80 {
        out.push((value & 0x7f) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

pub(super) fn var_int(value: i32) -> Vec<u8> {
    let mut out = Vec::new();
    var_uint(u64::from(((value << 1) ^ (value >> 31)) as u32), &mut out);
    out
}

/// Kiwi's 64-bit varint: seven bits per byte, a full eighth bit in the ninth.
pub(super) fn var_uint64(value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut rest = value;
    for _ in 0..8 {
        if rest < 0x80 {
            out.push(rest as u8);
            return out;
        }
        out.push((rest & 0x7f) as u8 | 0x80);
        rest >>= 7;
    }
    out.push(rest as u8);
    out
}

pub(super) fn var_int64(value: i64) -> Vec<u8> {
    var_uint64(((value << 1) ^ (value >> 63)) as u64)
}

pub(super) fn var_float(value: f32) -> Vec<u8> {
    if value == 0.0 {
        vec![0]
    } else {
        value.to_bits().rotate_right(23).to_le_bytes().to_vec()
    }
}

pub(super) fn string(text: &str) -> Vec<u8> {
    let mut out = text.as_bytes().to_vec();
    out.push(0);
    out
}

pub(super) fn field(name: &str, type_id: i32, is_array: bool, value: u32) -> Vec<u8> {
    let mut out = string(name);
    out.extend(var_int(type_id));
    out.push(u8::from(is_array));
    var_uint(u64::from(value), &mut out);
    out
}

pub(super) fn definition(name: &str, kind: u8, fields: &[Vec<u8>]) -> Vec<u8> {
    let mut out = string(name);
    out.push(kind);
    var_uint(fields.len() as u64, &mut out);
    for field in fields {
        out.extend(field);
    }
    out
}

pub(super) fn schema(definitions: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    var_uint(definitions.len() as u64, &mut out);
    for definition in definitions {
        out.extend(definition);
    }
    out
}

pub(super) const BOOL: i32 = -1;
pub(super) const BYTE: i32 = -2;
pub(super) const INT: i32 = -3;
pub(super) const UINT: i32 = -4;
pub(super) const FLOAT: i32 = -5;
pub(super) const STRING: i32 = -6;
pub(super) const INT64: i32 = -7;
pub(super) const UINT64: i32 = -8;

/// `Color` (enum, 0), `Point` (struct, 1) and `Msg` (message, 2).
pub(super) fn sample_schema() -> Vec<u8> {
    schema(&[
        definition(
            "Color",
            0,
            &[field("RED", 0, false, 1), field("GREEN", 0, false, 2)],
        ),
        definition(
            "Point",
            1,
            &[field("x", FLOAT, false, 1), field("y", FLOAT, false, 2)],
        ),
        definition(
            "Msg",
            2,
            &[
                field("id", UINT, false, 1),
                field("name", STRING, false, 2),
                field("tags", STRING, true, 3),
                field("color", 0, false, 4),
                field("origin", 1, false, 5),
                field("data", BYTE, true, 6),
                field("big", INT64, false, 7),
                field("flag", BOOL, false, 8),
                field("n", INT, false, 9),
                field("u", UINT64, false, 10),
                field("b", BYTE, false, 11),
                field("points", 1, true, 12),
                field("nested", 2, false, 13),
            ],
        ),
    ])
}
