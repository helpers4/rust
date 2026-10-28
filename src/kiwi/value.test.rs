// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn object() -> Value {
    Value::Object(vec![
        ("a".into(), Value::Uint(1)),
        ("b".into(), Value::String("text".into())),
    ])
}

#[test]
fn get_finds_a_field_of_an_object() {
    assert_eq!(object().get("a"), Some(&Value::Uint(1)));
    assert_eq!(object().get("b").and_then(Value::as_str), Some("text"));
    assert_eq!(object().get("c"), None);
}

#[test]
fn get_on_anything_but_an_object_is_none() {
    assert_eq!(Value::Uint(1).get("a"), None);
    assert_eq!(Value::Array(vec![]).get("a"), None);
}

#[test]
fn as_str_reads_strings_and_enum_names() {
    assert_eq!(Value::String("s".into()).as_str(), Some("s"));
    assert_eq!(Value::Enum("RED".into()).as_str(), Some("RED"));
    assert_eq!(Value::Uint(1).as_str(), None);
}

#[test]
fn the_scalar_accessors_only_read_their_own_type() {
    assert_eq!(Value::Bool(true).as_bool(), Some(true));
    assert_eq!(Value::Int(-1).as_i32(), Some(-1));
    assert_eq!(Value::Uint(1).as_u32(), Some(1));
    assert_eq!(Value::Int64(-2).as_i64(), Some(-2));
    assert_eq!(Value::Uint64(2).as_u64(), Some(2));
    assert_eq!(Value::Float(0.5).as_f32(), Some(0.5));
    assert_eq!(Value::Bytes(vec![1, 2]).as_bytes(), Some(&[1u8, 2][..]));

    let other = Value::Byte(0);
    assert_eq!(other.as_bool(), None);
    assert_eq!(other.as_i32(), None);
    assert_eq!(other.as_u32(), None);
    assert_eq!(other.as_i64(), None);
    assert_eq!(other.as_u64(), None);
    assert_eq!(other.as_f32(), None);
    assert_eq!(other.as_bytes(), None);
}

#[test]
fn as_array_is_empty_for_anything_but_an_array() {
    assert_eq!(
        Value::Array(vec![Value::Uint(1)]).as_array(),
        [Value::Uint(1)]
    );
    assert_eq!(Value::Uint(1).as_array(), []);
}
