// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::base32_encode::base32_encode;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn decoding_an_encoded_value_gives_it_back(data in prop::collection::vec(0u8..255, 0..64)) {
        prop_assert_eq!(base32_decode(&base32_encode(&data)), Ok(data));
    }

    #[test]
    fn never_panics(s in ".*") {
        let _ = base32_decode(&s);
    }
}
