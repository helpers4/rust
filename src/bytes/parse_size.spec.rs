// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_plain_number_of_bytes_reads_back(n in any::<u64>()) {
        prop_assert_eq!(parse_size(&n.to_string()), Ok(n));
    }

    #[test]
    fn reads_back_what_format_size_wrote_within_its_rounding(n in 0u64..u64::MAX / 2) {
        let back = parse_size(&crate::bytes::format_size(n)).unwrap();
        let (n, back) = (u128::from(n), u128::from(back));
        // One decimal of a unit is at most 5% of the value once it is at least 1 KiB.
        prop_assert!(n.abs_diff(back) * 20 <= n.max(1), "{n} vs {back}");
    }
}
