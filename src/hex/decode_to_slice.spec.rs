// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn agrees_with_decode(bytes in prop::collection::vec(any::<u8>(), 0..32)) {
        let hex = crate::hex::encode(&bytes);
        let mut out = vec![0u8; bytes.len()];
        decode_to_slice(&hex, &mut out).unwrap();
        prop_assert_eq!(out, bytes);
    }
}
