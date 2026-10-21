// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::bytes::zigzag_encode;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_through_encode(value in any::<u64>()) {
        prop_assert_eq!(zigzag_encode(zigzag_decode(value)), value);
    }

    #[test]
    fn a_32_bit_value_fits_in_i32(value in any::<u32>()) {
        prop_assert!(i32::try_from(zigzag_decode(u64::from(value))).is_ok());
    }
}
