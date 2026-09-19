// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_two_lowercase_hex_digits_per_byte(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        let out = encode(&bytes);
        prop_assert_eq!(out.len(), bytes.len() * 2);
        prop_assert!(out.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    }
}
