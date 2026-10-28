// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_support::*;
use super::*;

fn sample() -> Schema {
    Schema::parse(&sample_schema()).unwrap()
}

#[test]
fn parses_definitions_kinds_and_field_types() {
    let schema = sample();
    assert_eq!(schema.definitions.len(), 3);
    let color = &schema.definitions[0];
    assert_eq!((&*color.name, color.kind), ("Color", Kind::Enum));
    assert_eq!(color.fields[0].field_type, None);
    assert_eq!(color.fields[1].value, 2);
    let point = &schema.definitions[1];
    assert_eq!(point.kind, Kind::Struct);
    assert_eq!(point.fields[0].field_type, Some(FieldType::Float));
    let msg = &schema.definitions[2];
    assert_eq!(msg.kind, Kind::Message);
    assert_eq!(msg.fields[2].field_type, Some(FieldType::String));
    assert!(msg.fields[2].is_array);
    assert_eq!(msg.fields[3].field_type, Some(FieldType::Definition(0)));
    assert_eq!(msg.fields[10].value, 11);
}

#[test]
fn resolves_every_builtin_type() {
    let types = [
        (BOOL, FieldType::Bool),
        (BYTE, FieldType::Byte),
        (INT, FieldType::Int),
        (UINT, FieldType::Uint),
        (FLOAT, FieldType::Float),
        (STRING, FieldType::String),
        (INT64, FieldType::Int64),
        (UINT64, FieldType::Uint64),
    ];
    for (type_id, expected) in types {
        let bytes = schema(&[definition("S", 1, &[field("f", type_id, false, 1)])]);
        let parsed = Schema::parse(&bytes).unwrap();
        assert_eq!(parsed.definitions[0].fields[0].field_type, Some(expected));
    }
}

#[test]
fn finds_a_definition_by_name() {
    let schema = sample();
    assert_eq!(schema.find("Color"), Some(0));
    assert_eq!(schema.find("Msg"), Some(2));
    assert_eq!(schema.find("Missing"), None);
}

#[test]
fn rejects_an_unknown_kind() {
    let bytes = schema(&[definition("S", 7, &[])]);
    assert_eq!(
        Schema::parse(&bytes),
        Err(KiwiError::UnknownKind { kind: 7 })
    );
}

#[test]
fn rejects_an_unknown_type() {
    let past_the_end = schema(&[definition("S", 1, &[field("f", 5, false, 1)])]);
    assert_eq!(
        Schema::parse(&past_the_end),
        Err(KiwiError::UnknownType { type_id: 5 })
    );
    let below_builtins = schema(&[definition("S", 1, &[field("f", -9, false, 1)])]);
    assert_eq!(
        Schema::parse(&below_builtins),
        Err(KiwiError::UnknownType { type_id: -9 })
    );
}

#[test]
fn an_enum_member_type_is_not_checked() {
    let bytes = schema(&[definition("E", 0, &[field("A", 99, false, 0)])]);
    assert!(Schema::parse(&bytes).is_ok());
}

#[test]
fn rejects_truncated_and_invalid_schemas() {
    let bytes = sample_schema();
    for len in 0..bytes.len() {
        assert!(Schema::parse(&bytes[..len]).is_err(), "{len}");
    }
    assert_eq!(
        Schema::parse(&[1, 0xff, 0]),
        Err(KiwiError::InvalidString { index: 1 })
    );
}

#[test]
fn a_huge_definition_or_field_count_is_an_error_not_an_allocation() {
    assert!(Schema::parse(&[0xff, 0xff, 0xff, 0xff, 0x0f]).is_err());
    let mut bytes = string("S");
    bytes.push(2);
    bytes.extend([0xff, 0xff, 0xff, 0xff, 0x0f]);
    let mut all = vec![1];
    all.extend(bytes);
    assert!(Schema::parse(&all).is_err());
}

fn encode_msg() -> Vec<u8> {
    let mut out = Vec::new();
    out.push(1); // id
    var_uint(300, &mut out);
    out.push(2); // name
    out.extend(string("Frame 1"));
    out.push(3); // tags
    out.push(2);
    out.extend(string("a"));
    out.extend(string("b"));
    out.push(4); // color
    out.push(2);
    out.push(5); // origin
    out.extend(var_float(1.5));
    out.extend(var_float(0.0));
    out.push(6); // data
    out.extend([3, 9, 8, 7]);
    out.push(7); // big
    out.extend(var_int64(-5_000_000_000));
    out.push(8); // flag
    out.push(1);
    out.push(9); // n
    out.extend(var_int(-7));
    out.push(10); // u
    out.extend(var_uint64(u64::MAX));
    out.push(11); // b
    out.push(200);
    out.push(12); // points
    out.push(2);
    for (x, y) in [(1.0, 2.0), (-3.0, 0.5)] {
        out.extend(var_float(x));
        out.extend(var_float(y));
    }
    out.push(0);
    out
}

#[test]
fn decodes_every_kind_of_value() {
    let value = sample().decode(&encode_msg(), "Msg").unwrap();
    assert_eq!(value.get("id").and_then(Value::as_u32), Some(300));
    assert_eq!(value.get("name").and_then(Value::as_str), Some("Frame 1"));
    assert_eq!(
        value.get("tags"),
        Some(&Value::Array(vec![
            Value::String("a".into()),
            Value::String("b".into())
        ]))
    );
    assert_eq!(value.get("color").and_then(Value::as_str), Some("GREEN"));
    let origin = value.get("origin").unwrap();
    assert_eq!(origin.get("x").and_then(Value::as_f32), Some(1.5));
    assert_eq!(origin.get("y").and_then(Value::as_f32), Some(0.0));
    assert_eq!(
        value.get("data").and_then(Value::as_bytes),
        Some(&[9u8, 8, 7][..])
    );
    assert_eq!(
        value.get("big").and_then(Value::as_i64),
        Some(-5_000_000_000)
    );
    assert_eq!(value.get("flag").and_then(Value::as_bool), Some(true));
    assert_eq!(value.get("n").and_then(Value::as_i32), Some(-7));
    assert_eq!(value.get("u").and_then(Value::as_u64), Some(u64::MAX));
    assert_eq!(value.get("b"), Some(&Value::Byte(200)));
    let points = value.get("points").unwrap().as_array();
    assert_eq!(points.len(), 2);
    assert_eq!(points[1].get("x").and_then(Value::as_f32), Some(-3.0));
    assert_eq!(value.get("nested"), None);
}

#[test]
fn an_empty_message_has_no_field() {
    assert_eq!(sample().decode(&[0], "Msg"), Ok(Value::Object(vec![])));
}

#[test]
fn keeps_the_field_order_of_the_data() {
    let mut data = vec![2];
    data.extend(string("x"));
    data.push(1);
    data.push(5);
    data.push(0);
    let Value::Object(fields) = sample().decode(&data, "Msg").unwrap() else {
        panic!("expected an object");
    };
    let names: Vec<&str> = fields.iter().map(|(name, _)| &**name).collect();
    assert_eq!(names, ["name", "id"]);
}

#[test]
fn an_enum_number_missing_from_the_schema_is_kept() {
    let data = [4, 9, 0];
    assert_eq!(
        sample().decode(&data, "Msg").unwrap().get("color"),
        Some(&Value::UnknownEnum(9))
    );
}

#[test]
fn can_decode_a_struct_or_an_enum_as_the_root() {
    let mut point = var_float(2.0);
    point.extend(var_float(4.0));
    let value = sample().decode(&point, "Point").unwrap();
    assert_eq!(value.get("y").and_then(Value::as_f32), Some(4.0));
    assert_eq!(
        sample().decode(&[1], "Color"),
        Ok(Value::Enum("RED".into()))
    );
}

#[test]
fn leaves_trailing_bytes_alone() {
    assert!(sample().decode(&[0, 1, 2, 3], "Msg").is_ok());
}

#[test]
fn rejects_an_unknown_root() {
    assert_eq!(
        sample().decode(&[0], "Nope"),
        Err(KiwiError::UnknownDefinition {
            name: "Nope".into()
        })
    );
}

#[test]
fn rejects_a_field_id_the_message_does_not_have() {
    assert_eq!(
        sample().decode(&[99, 0], "Msg"),
        Err(KiwiError::UnknownField {
            message: "Msg".into(),
            id: 99
        })
    );
}

#[test]
fn reports_truncated_data_with_its_offset() {
    let data = encode_msg();
    let schema = sample();
    for len in 0..data.len() {
        assert!(schema.decode(&data[..len], "Msg").is_err(), "{len}");
    }
    assert_eq!(
        schema.decode(&[1], "Msg"),
        Err(KiwiError::UnexpectedEof { index: 1 })
    );
}

#[test]
fn a_huge_array_length_is_an_error_not_an_allocation() {
    let data = [3, 0xff, 0xff, 0xff, 0xff, 0x0f];
    assert!(sample().decode(&data, "Msg").is_err());
    let bytes = [6, 0xff, 0xff, 0xff, 0xff, 0x0f];
    assert_eq!(
        sample().decode(&bytes, "Msg"),
        Err(KiwiError::UnexpectedEof { index: 6 })
    );
}

#[test]
fn stops_at_the_nesting_limit() {
    // `nested` is a Msg inside a Msg: field 13, 100 levels deep.
    let data = [13; 100];
    assert_eq!(sample().decode(&data, "Msg"), Err(KiwiError::TooDeep));
    // A few levels are fine.
    let mut shallow = vec![13, 13, 13, 0, 0, 0, 0];
    assert!(sample().decode(&shallow, "Msg").is_ok());
    shallow.truncate(2);
    assert!(sample().decode(&shallow, "Msg").is_err());
}

#[test]
fn a_definition_index_out_of_range_is_an_unknown_type() {
    let schema = Schema {
        definitions: vec![Definition {
            name: "S".into(),
            kind: Kind::Struct,
            fields: vec![Field {
                name: "f".into(),
                field_type: Some(FieldType::Definition(9)),
                is_array: false,
                value: 1,
            }],
        }],
    };
    assert_eq!(
        schema.decode(&[], "S"),
        Err(KiwiError::UnknownType { type_id: 9 })
    );
}

#[test]
fn a_hand_built_field_without_a_type_reads_as_a_uint() {
    let schema = Schema {
        definitions: vec![Definition {
            name: "S".into(),
            kind: Kind::Struct,
            fields: vec![Field {
                name: "f".into(),
                field_type: None,
                is_array: false,
                value: 1,
            }],
        }],
    };
    assert_eq!(
        schema.decode(&[7], "S").unwrap().get("f"),
        Some(&Value::Uint(7))
    );
}
