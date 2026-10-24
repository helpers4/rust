// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Malformed input must reject with `ParseEasingError`, never panic or loop.
use helpers4::animation::Easing;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(easing) = Easing::parse(s) {
            let _ = easing.at(0.5);
        }
    }
});
