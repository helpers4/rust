// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn well_formed_names_with_an_alphabetic_last_label_are_accepted(
        labels in prop::collection::vec("[a-z0-9]([a-z0-9-]{0,10}[a-z0-9])?", 0..4),
        last in "[a-z]{1,10}",
    ) {
        let name = labels.iter().chain(std::iter::once(&last)).cloned().collect::<Vec<_>>().join(".");
        prop_assert_eq!(is_valid_hostname(&name), Ok(()));
    }

    #[test]
    fn a_decimal_last_label_is_always_rejected(
        prefix in "([a-z]{1,5}\\.){0,3}",
        number in "[0-9]{1,10}",
    ) {
        prop_assert_eq!(
            is_valid_hostname(&format!("{prefix}{number}")),
            Err(HostnameError::NumericLastLabel)
        );
    }

    #[test]
    fn a_trailing_dot_never_changes_the_verdict(name in "[a-z0-9.-]{1,40}") {
        prop_assume!(!name.ends_with('.'));
        prop_assert_eq!(is_valid_hostname(&format!("{name}.")), is_valid_hostname(&name));
    }

    #[test]
    fn never_panics_and_accepted_names_are_ascii(s in ".*") {
        if is_valid_hostname(&s).is_ok() {
            prop_assert!(s.is_ascii());
        }
    }
}
