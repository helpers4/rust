// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_on_any_bytes(bytes in prop::collection::vec(any::<u8>(), 0..128)) {
        let _ = read_font(&bytes);
    }

    #[test]
    fn never_panics_on_a_damaged_font(position in 0usize..400, value in any::<u8>()) {
        use super::super::_test_support::*;
        let mut bytes = sfnt(&typical_tables("Inter", "Bold", 700, 0));
        let at = position % bytes.len();
        bytes[at] = value;
        let _ = read_font(&bytes);
    }

    #[test]
    fn reads_back_any_family_and_weight(family in "[A-Za-z][A-Za-z ]{0,16}", weight in 1u16..=1000) {
        use super::super::_test_support::*;
        let info = read_font(&sfnt(&typical_tables(&family, "Regular", weight, 0))).unwrap();
        prop_assert_eq!(info.family, family);
        prop_assert_eq!(info.weight, f64::from(weight));
    }
}
