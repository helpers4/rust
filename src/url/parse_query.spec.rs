// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn reads_back_what_build_query_wrote(
        pairs in prop::collection::vec(("\\PC{0,8}", "\\PC{0,8}"), 0..5),
    ) {
        let non_empty: Vec<(String, String)> = pairs
            .into_iter()
            .filter(|(k, v)| !(k.is_empty() && v.is_empty()))
            .collect();
        let query = crate::url::build_query(non_empty.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        prop_assert_eq!(parse_query(&query).unwrap(), non_empty);
    }
}
