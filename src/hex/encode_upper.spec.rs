// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_the_uppercase_of_encode(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        prop_assert_eq!(encode_upper(&bytes), crate::hex::encode(&bytes).to_uppercase());
    }
}
