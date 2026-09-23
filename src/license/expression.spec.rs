// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn id() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("MIT"),
        Just("Apache-2.0"),
        Just("ISC"),
        Just("GPL-2.0-only"),
        Just("BSD-3-Clause")
    ]
    .prop_map(str::to_string)
}

fn expression() -> impl Strategy<Value = String> {
    id().prop_recursive(3, 12, 2, |inner| {
        prop_oneof![
            (inner.clone(), inner.clone()).prop_map(|(a, b)| format!("({a} OR {b})")),
            (inner.clone(), inner).prop_map(|(a, b)| format!("({a} AND {b})")),
        ]
    })
}

proptest! {
    #[test]
    fn display_then_parse_gives_the_same_expression(text in expression()) {
        let parsed = Expression::parse(&text).unwrap();
        prop_assert_eq!(Expression::parse(&parsed.to_string()), Ok(parsed));
    }

    #[test]
    fn allowing_every_license_always_satisfies(text in expression()) {
        let parsed = Expression::parse(&text).unwrap();
        let all = parsed.licenses();
        prop_assert!(parsed.is_satisfied_by(&all));
        prop_assert!(!parsed.is_satisfied_by(&[]));
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,30}") {
        let _ = Expression::parse(&s);
    }
}
