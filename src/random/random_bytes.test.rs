// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn returns_the_requested_length() {
    for len in [0, 1, 7, 32, 1000] {
        assert_eq!(random_bytes(len).map(|b| b.len()), Ok(len));
    }
}

#[test]
fn two_draws_differ() {
    assert_ne!(random_bytes(32), random_bytes(32)); // equal with probability 2^-256
}
