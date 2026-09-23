// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_failure_anywhere_gives_failure(pos in 0usize..5, n in 1usize..6) {
        let mut statuses = vec![Status::Success; n];
        statuses[pos % n] = Status::Failure;
        prop_assert_eq!(overall(&statuses), Status::Failure);
    }
}
