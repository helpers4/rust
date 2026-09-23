// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn repeats_every_400_years(year in -5000i32..5000) {
        prop_assert_eq!(is_leap_year(year), is_leap_year(year + 400));
    }
}
