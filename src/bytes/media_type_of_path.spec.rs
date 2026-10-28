// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(path in "\\PC{0,40}") {
        let _ = media_type_of_path(&path);
    }

    #[test]
    fn ignores_the_folders_the_query_and_the_case_of_a_known_extension(
        dir in "[a-z0-9/]{0,12}",
        extension in prop::sample::select(vec!["png", "PNG", "Png"]),
        query in "[a-z0-9=&]{0,8}",
    ) {
        prop_assert_eq!(
            media_type_of_path(&format!("{dir}/file.{extension}?{query}")),
            Some("image/png")
        );
    }
}
