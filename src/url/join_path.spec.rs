// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_exactly_one_slash_at_the_seam(a in "[a-z]{1,5}/{0,3}", b in "/{0,3}[a-z]{1,5}") {
        let joined = join_path(&a, &b);
        prop_assert_eq!(joined.clone(), format!("{}/{}", a.trim_end_matches('/'), b.trim_start_matches('/')));
        prop_assert!(!joined.contains("//"));
    }
}
