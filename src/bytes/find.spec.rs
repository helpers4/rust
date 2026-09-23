// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn agrees_with_str_find_on_ascii(h in "[a-c]{0,12}", n in "[a-c]{0,3}") {
        prop_assert_eq!(find(h.as_bytes(), n.as_bytes()), h.find(&n));
    }
}
