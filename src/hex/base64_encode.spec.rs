// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn output_length_is_always_a_multiple_of_four(data in prop::collection::vec(0u8..255, 0..64)) {
        let encoded = base64_encode(&data);
        prop_assert_eq!(encoded.len(), data.len().div_ceil(3) * 4);
        prop_assert_eq!(encoded.len() % 4, 0);
    }
}
