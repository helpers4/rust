// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent(segments in prop::collection::vec("(a|b|\\.|\\.\\.)", 0..8)) {
        let path = PathBuf::from(segments.join("/"));
        let once = normalize(&path);
        prop_assert_eq!(normalize(&once), once);
    }
}
