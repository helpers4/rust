// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_with_the_std_conversions(value in any::<u64>(), pad in 0usize..5) {
        let mut buf = vec![0u8; pad];
        buf.extend_from_slice(&value.to_be_bytes());
        prop_assert_eq!(read_u64(&buf, pad, Endian::Big), Some(value));
        let mut buf = vec![0u8; pad];
        buf.extend_from_slice(&value.to_le_bytes());
        prop_assert_eq!(read_u64(&buf, pad, Endian::Little), Some(value));
    }
}
