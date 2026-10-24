// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn display_then_parse_gives_back_the_same_id_for_any_128_bits(value in any::<u128>()) {
        let text = Ulid::from_u128(value).to_string();
        prop_assert_eq!(text.len(), 26);
        prop_assert_eq!(Ulid::parse(&text), Ok(Ulid::from_u128(value)));
        prop_assert_eq!(Ulid::parse(&text.to_lowercase()), Ok(Ulid::from_u128(value)));
    }

    #[test]
    fn from_parts_round_trips_the_timestamp(ms in 0u64..(1 << 48), random in any::<[u8; 10]>()) {
        prop_assert_eq!(Ulid::from_parts(ms, random).timestamp_millis(), ms);
    }

    #[test]
    fn text_order_follows_numeric_order(a in any::<u128>(), b in any::<u128>()) {
        let (x, y) = (Ulid::from_u128(a), Ulid::from_u128(b));
        prop_assert_eq!(x.to_string().cmp(&y.to_string()), a.cmp(&b));
    }

    #[test]
    fn parse_never_panics(text in ".{0,40}") {
        let _ = Ulid::parse(&text);
    }
}
