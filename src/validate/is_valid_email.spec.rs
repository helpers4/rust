// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn accepts_any_well_formed_local_and_domain(
        local in "[a-z0-9]{1,10}",
        sub in "[a-z0-9]{1,10}",
        tld in "[a-z]{2,6}",
    ) {
        let address = std::format!("{local}@{sub}.{tld}");
        prop_assert!(is_valid_email(&address));
    }

    #[test]
    fn never_accepts_input_without_an_at_sign(s in "[a-z0-9.]{0,30}") {
        prop_assert!(!is_valid_email(&s));
    }
}
