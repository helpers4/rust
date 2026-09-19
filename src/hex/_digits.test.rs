// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn encodes_lower_and_upper() {
    assert_eq!(encode_with(&[0x00, 0xff, 0x1a], false), "00ff1a");
    assert_eq!(encode_with(&[0x00, 0xff, 0x1a], true), "00FF1A");
    assert_eq!(encode_with(&[], false), "");
}
