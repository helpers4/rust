// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn an_exact_requirement_matches_only_that_version(
        major in 0u64..5, minor in 0u64..5, patch in 0u64..5,
        other_major in 0u64..5, other_minor in 0u64..5, other_patch in 0u64..5,
    ) {
        let req = VersionReq::parse(&format!("={major}.{minor}.{patch}")).unwrap();
        let other = Version::new(other_major, other_minor, other_patch);
        prop_assert_eq!(req.matches(&other), (major, minor, patch) == (other_major, other_minor, other_patch));
    }

    #[test]
    fn caret_accepts_the_version_itself_and_never_a_lower_one(
        major in 0u64..5, minor in 0u64..5, patch in 0u64..5,
        other_major in 0u64..5, other_minor in 0u64..5, other_patch in 0u64..5,
    ) {
        let req = VersionReq::parse(&format!("^{major}.{minor}.{patch}")).unwrap();
        prop_assert!(req.matches(&Version::new(major, minor, patch)));
        let other = Version::new(other_major, other_minor, other_patch);
        if other < Version::new(major, minor, patch) {
            prop_assert!(!req.matches(&other));
        }
    }

    #[test]
    fn a_range_matches_exactly_the_versions_between_its_bounds(
        low in 0u64..6, high in 0u64..6, candidate in 0u64..6,
    ) {
        let req = VersionReq::parse(&format!(">={low}.0.0, <{high}.0.0")).unwrap();
        prop_assert_eq!(req.matches(&Version::new(candidate, 3, 1)), candidate >= low && candidate < high);
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,20}") {
        let _ = VersionReq::parse(&s);
    }
}
