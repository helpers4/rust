// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_support::*;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_and_decode_never_panic_on_any_bytes(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        let _ = Schema::parse(&bytes);
        let schema = Schema::parse(&sample_schema()).unwrap();
        let _ = schema.decode(&bytes, "Msg");
        let _ = schema.decode(&bytes, "Point");
    }

    #[test]
    fn a_message_of_scalars_decodes_to_what_was_encoded(
        id in any::<u32>(),
        n in any::<i32>(),
        big in any::<i64>(),
        u in any::<u64>(),
        flag in any::<bool>(),
        name in "[a-zA-Z0-9 ]{0,12}",
    ) {
        let mut data = vec![1];
        var_uint(u64::from(id), &mut data);
        data.push(2);
        data.extend(string(&name));
        data.push(7);
        data.extend(var_int64(big));
        data.push(8);
        data.push(u8::from(flag));
        data.push(9);
        data.extend(var_int(n));
        data.push(10);
        data.extend(var_uint64(u));
        data.push(0);
        let schema = Schema::parse(&sample_schema()).unwrap();
        let value = schema.decode(&data, "Msg").unwrap();
        prop_assert_eq!(value.get("id").and_then(Value::as_u32), Some(id));
        prop_assert_eq!(value.get("name").and_then(Value::as_str), Some(name.as_str()));
        prop_assert_eq!(value.get("big").and_then(Value::as_i64), Some(big));
        prop_assert_eq!(value.get("flag").and_then(Value::as_bool), Some(flag));
        prop_assert_eq!(value.get("n").and_then(Value::as_i32), Some(n));
        prop_assert_eq!(value.get("u").and_then(Value::as_u64), Some(u));
    }
}
