// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn get_reads_back_exactly_what_set_wrote(
        content in ".*",
        key in "[A-Za-z_][A-Za-z0-9_]{0,8}",
        value in ".*",
    ) {
        let updated = set(&content, &key, &value).unwrap();
        prop_assert_eq!(crate::env::get(&updated, &key), Some(value));
    }

    #[test]
    fn is_idempotent(
        content in ".*",
        key in "[A-Za-z_][A-Za-z0-9_]{0,8}",
        value in ".*",
    ) {
        let once = set(&content, &key, &value).unwrap();
        prop_assert_eq!(set(&once, &key, &value).unwrap(), once);
    }

    #[test]
    fn leaves_other_variables_alone(
        content in "([A-C]=[a-z0-9]{0,4}\n){0,6}",
        value in "[a-z ]{0,6}",
    ) {
        let updated = set(&content, "Z", &value).unwrap();
        let others = |c: &str| crate::env::parse(c).into_iter().filter(|(k, _)| k != "Z").collect::<Vec<_>>();
        prop_assert_eq!(others(&updated), others(&content));
    }
}
