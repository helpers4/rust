// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn anything_built_from_plain_segments_is_inside(
        segments in prop::collection::vec("[a-z]{1,4}", 0..5),
    ) {
        let path = std::path::PathBuf::from(segments.join("/"));
        prop_assert!(is_within(Path::new("/base"), &path));
    }

    #[test]
    fn a_path_that_starts_with_a_parent_dir_is_outside(rest in "[a-z]{1,4}(/[a-z]{1,4}){0,3}") {
        prop_assert!(!is_within(Path::new("/base"), &Path::new("..").join(rest)));
    }
}
