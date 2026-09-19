// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn encodes_lowercase_with_two_digits_per_byte() {
    assert_eq!(encode(&[0x00, 0x0f, 0xa0, 0xff]), "000fa0ff");
}

#[test]
fn empty_input_gives_empty_string() {
    assert_eq!(encode(&[]), "");
}
