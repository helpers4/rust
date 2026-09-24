// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// RFC 3986 parsing has the most branches of any parser in the crate (schemes, authority,
// percent-encoding, reference resolution): malformed input must reject with `UrlError`, never
// panic.
use helpers4::url::Url;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Url::parse(s);
    }
});
