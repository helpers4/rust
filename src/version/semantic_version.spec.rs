// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn any_version() -> impl Strategy<Value = Version> {
    (
        0u64..100,
        0u64..100,
        0u64..100,
        proptest::option::of("[a-z]{1,3}(\\.[0-9a-z]{1,2}){0,2}"),
    )
        .prop_map(|(major, minor, patch, pre)| {
            let text = match pre {
                Some(pre) => format!("{major}.{minor}.{patch}-{pre}"),
                None => format!("{major}.{minor}.{patch}"),
            };
            Version::parse(&text).unwrap_or_else(|_| Version::new(major, minor, patch))
        })
}

proptest! {
    #[test]
    fn display_and_parse_round_trip(version in any_version()) {
        prop_assert_eq!(Version::parse(&version.to_string()), Ok(version));
    }

    #[test]
    fn ordering_is_total_and_antisymmetric(a in any_version(), b in any_version()) {
        prop_assert_eq!(a.cmp(&b), b.cmp(&a).reverse());
        prop_assert_eq!(a == b, a.cmp(&b) == Ordering::Equal);
    }

    #[test]
    fn ordering_is_transitive(a in any_version(), b in any_version(), c in any_version()) {
        if a <= b && b <= c {
            prop_assert!(a <= c);
        }
    }

    #[test]
    fn a_bump_is_always_greater(version in any_version()) {
        prop_assert!(version.bump_patch() > version);
        prop_assert!(version.bump_minor() > version);
        prop_assert!(version.bump_major() > version);
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,20}") {
        let _ = Version::parse(&s);
        let _ = Version::parse_lenient(&s);
    }
}
