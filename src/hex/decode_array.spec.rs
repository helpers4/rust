// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_with_encode(bytes in any::<[u8; 8]>()) {
        let hex = crate::hex::encode(&bytes);
        prop_assert_eq!(decode_array::<8>(&hex).unwrap(), bytes);
        prop_assert!(decode_array::<8>(&hex[..hex.len() - 2]).is_err());
    }
}
