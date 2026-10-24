// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Every read on arbitrary bytes must return `None` at the end of the data, never panic, and the
// reader must never move past the end of the slice.
use helpers4::bytes::{ByteReader, Endian};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    for endian in [Endian::Big, Endian::Little] {
        let mut reader = ByteReader::new(data);
        while reader.remaining() > 0 {
            let before = reader.position();
            let _ = reader.read_u8();
            let _ = reader.read_u16(endian);
            let _ = reader.read_i16(endian);
            let _ = reader.read_u32(endian);
            let _ = reader.read_i32(endian);
            let _ = reader.read_u64(endian);
            let _ = reader.read_i64(endian);
            let _ = reader.read_f32(endian);
            let _ = reader.read_f64(endian);
            let _ = reader.read_var_u32();
            let _ = reader.read_var_u64();
            let _ = reader.read_var_i32();
            let _ = reader.read_var_i64();
            let _ = reader.read_cstr();
            let _ = reader.read_bytes(usize::MAX);
            assert!(reader.position() <= data.len());
            if reader.position() == before {
                break;
            }
        }
    }
});
