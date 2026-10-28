// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_symmetric_and_reflexive(a in "[A-Za-z ]{0,12}", b in "[A-Za-z ]{0,12}") {
        prop_assert!(same_family(&a, &a));
        prop_assert_eq!(same_family(&a, &b), same_family(&b, &a));
    }

    #[test]
    fn case_and_spaces_never_matter(a in "[A-Za-z]{0,12}") {
        prop_assert!(same_family(&a, &a.to_uppercase()));
        let padded = format!(" {a} ");
        prop_assert!(same_family(&a, &padded));
    }
}
