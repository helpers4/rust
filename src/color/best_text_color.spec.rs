// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_result_contrasts_at_least_4_point_5_or_is_the_best_available(
        r in any::<u8>(), g in any::<u8>(), b in any::<u8>(),
    ) {
        let background = Rgb::new(r, g, b);
        let text = best_text_color(background);
        // Black or white always reaches at least sqrt(21) ~ 4.58 on any background.
        prop_assert!(contrast_ratio(background, text) > 4.5);
    }
}
