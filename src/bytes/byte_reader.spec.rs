// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::bytes::zigzag_encode;
use proptest::prelude::*;

fn leb128(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let byte = u8::try_from(value & 0x7f).unwrap_or(0);
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return out;
        }
        out.push(byte | 0x80);
    }
}

proptest! {
    #[test]
    fn var_u64_round_trips(value in any::<u64>()) {
        let bytes = leb128(value);
        let mut reader = ByteReader::new(&bytes);
        prop_assert_eq!(reader.read_var_u64(), Some(value));
        prop_assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn var_u32_round_trips(value in any::<u32>()) {
        let bytes = leb128(u64::from(value));
        prop_assert_eq!(ByteReader::new(&bytes).read_var_u32(), Some(value));
    }

    #[test]
    fn var_i32_round_trips(value in any::<i32>()) {
        let bytes = leb128(zigzag_encode(i64::from(value)));
        prop_assert_eq!(ByteReader::new(&bytes).read_var_i32(), Some(value));
    }

    #[test]
    fn var_i64_round_trips(value in any::<i64>()) {
        let bytes = leb128(zigzag_encode(value));
        prop_assert_eq!(ByteReader::new(&bytes).read_var_i64(), Some(value));
    }

    #[test]
    fn a_truncated_varint_fails_without_moving(value in 128u64.., cut in 1usize..10) {
        let bytes = leb128(value);
        let cut = cut.min(bytes.len() - 1);
        let mut reader = ByteReader::new(&bytes[..cut]);
        prop_assert_eq!(reader.read_var_u64(), None);
        prop_assert_eq!(reader.position(), 0);
    }

    #[test]
    fn never_panics_and_position_never_passes_the_end(data in proptest::collection::vec(any::<u8>(), 0..64), ops in proptest::collection::vec(0u8..8, 0..32)) {
        let mut reader = ByteReader::new(&data);
        for op in ops {
            match op {
                0 => { let _ = reader.read_u8(); }
                1 => { let _ = reader.read_u32(Endian::Big); }
                2 => { let _ = reader.read_var_u32(); }
                3 => { let _ = reader.read_var_u64(); }
                4 => { let _ = reader.read_var_i32(); }
                5 => { let _ = reader.read_cstr(); }
                6 => { let _ = reader.read_bytes(3); }
                _ => { let _ = reader.read_f64(Endian::Little); }
            }
            prop_assert!(reader.position() <= data.len());
            prop_assert_eq!(reader.position() + reader.remaining(), data.len());
        }
    }
}
