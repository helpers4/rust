// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_with_encode(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        prop_assert_eq!(decode(&crate::hex::encode(&bytes)).unwrap(), bytes.clone());
        prop_assert_eq!(decode(&crate::hex::encode_upper(&bytes)).unwrap(), bytes);
    }

    #[test]
    fn never_panics_on_arbitrary_input(s in ".*") {
        let _ = decode(&s);
    }
}
