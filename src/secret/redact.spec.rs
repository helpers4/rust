// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_secret_no_longer_appears(text in "[a-z ]{0,30}", secret in "[a-z]{3,6}") {
        let out = redact(&format!("{text} {secret} {text}"), &[&secret]);
        prop_assert!(out.contains(REDACTED));
        // When `text` has no copy of the secret, the only one is the middle one, and the spaces
        // around it keep the pieces from joining into a new one once the placeholder is gone.
        if !text.contains(&secret) {
            prop_assert!(!out.replace(REDACTED, "").contains(&secret));
        }
    }
}
