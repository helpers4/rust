// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips_any_valid_token(token in "[A-Za-z0-9._~+/-]{1,40}={0,3}") {
        let header = format!("Bearer {token}");
        prop_assert_eq!(bearer_token(&header), Some(token.as_str()));
    }

    #[test]
    fn never_panics_and_only_returns_a_slice_of_the_input(header in ".*") {
        if let Some(token) = bearer_token(&header) {
            prop_assert!(header.contains(token));
            prop_assert!(!token.is_empty());
        }
    }
}
