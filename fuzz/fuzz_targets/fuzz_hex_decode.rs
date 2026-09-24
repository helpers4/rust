// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Malformed input (odd length, non-hex characters) must reject with `DecodeError`, never panic.
use helpers4::hex::decode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = decode(s);
    }
});
