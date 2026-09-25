// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(content in ".*", key in "[A-Za-z_][A-Za-z0-9_]*") {
        let _ = get_duration(&content, &key, Duration::ZERO);
    }

    #[test]
    fn a_plain_number_of_seconds_round_trips(key in "[A-Za-z_][A-Za-z0-9_]*", secs in 0u64..100_000) {
        let content = format!("{key}={secs}s");
        prop_assert_eq!(get_duration(&content, &key, Duration::ZERO), Duration::from_secs(secs));
    }
}
