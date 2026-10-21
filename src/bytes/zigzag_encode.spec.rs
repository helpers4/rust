// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::bytes::zigzag_decode;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_through_decode(value in any::<i64>()) {
        prop_assert_eq!(zigzag_decode(zigzag_encode(value)), value);
    }

    #[test]
    fn smaller_magnitudes_encode_smaller(value in -1_000_000i64..1_000_000) {
        prop_assert!(zigzag_encode(value) <= 2 * value.unsigned_abs());
    }
}
