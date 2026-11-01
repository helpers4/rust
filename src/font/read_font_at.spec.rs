// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_on_any_bytes(bytes in prop::collection::vec(any::<u8>(), 0..128), index in 0usize..4) {
        let _ = read_font_at(&bytes, index);
    }

    #[test]
    fn never_panics_on_a_damaged_collection(position in 0usize..600, value in any::<u8>(), index in 0usize..3) {
        use super::super::_test_support::*;
        let tables = typical_tables("Inter", "Bold", 700, 0);
        let mut bytes = collection_of(&[&tables, &tables]);
        let at = position % bytes.len();
        bytes[at] = value;
        let _ = read_font_at(&bytes, index);
    }

    #[test]
    fn index_zero_is_what_read_font_reads(position in 0usize..400, value in any::<u8>()) {
        use super::super::_test_support::*;
        let mut bytes = sfnt(&typical_tables("Inter", "Bold", 700, 0));
        let at = position % bytes.len();
        bytes[at] = value;
        prop_assert_eq!(read_font_at(&bytes, 0), super::super::read_font(&bytes));
    }
}
