// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn removed_key_is_gone_and_others_stay(
        content in "([A-C]=[a-z0-9]{0,4}\n){0,8}",
    ) {
        let out = remove(&content, "A");
        prop_assert_eq!(crate::env::get(&out, "A"), None);
        let others = |c: &str| crate::env::parse(c).into_iter().filter(|(k, _)| k != "A").collect::<Vec<_>>();
        prop_assert_eq!(others(&out), others(&content));
    }
}
