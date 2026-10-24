// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Malformed input must reject with `ParseUlidError`, never panic; a parsed id writes back to itself.
use helpers4::id::Ulid;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(id) = Ulid::parse(s) {
            assert_eq!(Ulid::parse(&id.to_string()), Ok(id));
        }
    }
});
