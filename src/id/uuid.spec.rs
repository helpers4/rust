// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn display_then_parse_gives_back_the_same_id(bytes in any::<[u8; 16]>()) {
        let id = Uuid::from_bytes(bytes);
        prop_assert_eq!(Uuid::parse(&id.to_string()), Ok(id));
        prop_assert_eq!(id.to_string().len(), 36);
    }

    #[test]
    fn the_simple_form_parses_to_the_same_id(bytes in any::<[u8; 16]>()) {
        let id = Uuid::from_bytes(bytes);
        let simple = id.to_string().replace('-', "");
        prop_assert_eq!(Uuid::parse(&simple), Ok(id));
        prop_assert_eq!(Uuid::parse(&simple.to_uppercase()), Ok(id));
    }

    #[test]
    fn v4_always_has_version_4_and_the_rfc_variant(random in any::<[u8; 16]>()) {
        let id = Uuid::v4(random);
        prop_assert_eq!(id.version(), 4);
        prop_assert_eq!(id.as_bytes()[8] >> 6, 0b10);
    }

    #[test]
    fn v7_round_trips_the_timestamp_and_orders_by_it(a in 0u64..(1 << 48), b in 0u64..(1 << 48), ra in any::<[u8; 10]>(), rb in any::<[u8; 10]>()) {
        let (x, y) = (Uuid::v7(a, ra), Uuid::v7(b, rb));
        prop_assert_eq!(x.timestamp_millis(), Some(a));
        if a < b {
            prop_assert!(x < y);
        }
    }

    #[test]
    fn parse_never_panics(text in ".{0,50}") {
        let _ = Uuid::parse(&text);
    }
}
