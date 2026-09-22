// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn accepts_any_combination_of_hex_digits_in_the_canonical_shape(
        parts in prop::collection::vec("[0-9a-fA-F]{1,12}", 5..6),
    ) {
        let hex: Vec<String> = parts
            .into_iter()
            .zip([8, 4, 4, 4, 12])
            .map(|(p, len)| {
                let mut p = p;
                p.truncate(len);
                while p.len() < len { p.push('0'); }
                p
            })
            .collect();
        prop_assert!(is_uuid(&hex.join("-")));
    }

    #[test]
    fn a_wrong_length_is_always_rejected(s in "[0-9a-fA-F-]{0,50}") {
        if s.len() != 36 {
            prop_assert!(!is_uuid(&s));
        }
    }
}
