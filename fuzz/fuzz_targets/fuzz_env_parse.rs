// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// `env::parse` never returns a `Result` (a malformed line is skipped, not an error), so there is
// nothing to assert beyond "does not panic" on arbitrary `.env` content.
use helpers4::env::parse;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse(s);
    }
});
