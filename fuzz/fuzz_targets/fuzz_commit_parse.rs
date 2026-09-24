// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![no_main]

// Conventional Commits messages are untrusted input wherever a CI job parses PR commits: malformed
// input must reject with `ParseCommitError`, never panic.
use helpers4::commit::Commit;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Commit::parse(s);
    }
});
