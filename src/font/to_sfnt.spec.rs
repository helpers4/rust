// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_on_any_bytes(bytes in prop::collection::vec(any::<u8>(), 0..128)) {
        let _ = to_sfnt(&bytes);
    }

    #[test]
    fn never_panics_on_a_damaged_woff(position in 0usize..300, value in any::<u8>()) {
        use super::super::_test_support::*;
        let mut bytes = woff(&typical_tables("Inter", "Bold", 700, 0));
        let at = position % bytes.len();
        bytes[at] = value;
        let _ = to_sfnt(&bytes);
    }

    #[test]
    fn a_rebuilt_woff_reads_like_the_woff(family in "[A-Za-z][A-Za-z ]{0,16}", weight in 1u16..=1000) {
        use super::super::_test_support::*;
        let bytes = woff(&typical_tables(&family, "Regular", weight, 0));
        let sfnt = to_sfnt(&bytes).unwrap();
        prop_assert_eq!(super::super::read_font(&sfnt), super::super::read_font(&bytes));
    }
}
