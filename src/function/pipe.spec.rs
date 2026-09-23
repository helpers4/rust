// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_compose_with_the_arguments_swapped(n in -1000i32..1000) {
        let piped = pipe(|x: i32| x + 3, |x: i32| x * 2);
        let composed = crate::function::compose(|x: i32| x * 2, |x: i32| x + 3);
        prop_assert_eq!(piped(n), composed(n));
    }
}
