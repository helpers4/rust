// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn always_has_a_known_unit_and_a_number_below_1024(bytes in any::<u64>()) {
        let text = format_size(bytes);
        let (number, unit) = text.split_once(' ').unwrap();
        prop_assert!(["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"].contains(&unit));
        prop_assert!(number.parse::<f64>().unwrap() <= 1024.0);
        prop_assert!(!number.ends_with(".0"));
    }
}
