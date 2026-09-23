// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn always_returns_what_the_function_returns(args in prop::collection::vec(0u32..20, 0..30)) {
        let mut memo = Memoize::new(|n: &u32| n.pow(2));
        for arg in &args {
            prop_assert_eq!(memo.call(*arg), arg.pow(2));
        }
        let distinct: std::collections::HashSet<_> = args.iter().collect();
        prop_assert_eq!(memo.len(), distinct.len());
    }
}
