// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Malformed input must reject with `ParseColorError`, never panic; a parsed color writes back to itself.
use helpers4::color::Rgba;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(color) = Rgba::parse(s) {
            assert_eq!(Rgba::parse(&color.to_hex()), Ok(color));
        }
    }
});
