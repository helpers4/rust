// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_runs_more_than_the_attempts_allowed(attempts in 0u32..10, succeed_on in 0u32..12) {
        let mut calls = 0u32;
        let result: Result<u32, u32> = retry(attempts, |attempt| {
            calls += 1;
            if attempt == succeed_on { Ok(attempt) } else { Err(attempt) }
        });
        prop_assert!(calls <= attempts.max(1));
        match result {
            Ok(n) => prop_assert_eq!(n, calls),
            Err(last) => prop_assert_eq!(last, calls),
        }
    }
}
