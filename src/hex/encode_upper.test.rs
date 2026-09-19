// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn encodes_uppercase() {
    assert_eq!(encode_upper(&[0x00, 0x0f, 0xa0, 0xff]), "000FA0FF");
    assert_eq!(encode_upper(&[]), "");
}
