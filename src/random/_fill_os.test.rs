// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn fills_the_whole_buffer_with_different_bytes_each_time() {
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    assert!(fill_os(&mut a).is_ok());
    assert!(fill_os(&mut b).is_ok());
    // An all-zero or repeated 256-bit draw has a probability of 2^-256.
    assert_ne!(a, [0u8; 32]);
    assert_ne!(a, b);
}

#[test]
fn an_empty_buffer_is_fine() {
    assert!(fill_os(&mut []).is_ok());
}
