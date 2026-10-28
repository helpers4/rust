// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_and_answers_an_image_type_or_nothing(bytes in prop::collection::vec(any::<u8>(), 0..64)) {
        if let Some(media_type) = media_type_of(&bytes) {
            prop_assert!(media_type.starts_with("image/"));
        }
    }

    #[test]
    fn a_known_signature_wins_whatever_follows(tail in prop::collection::vec(any::<u8>(), 0..32)) {
        let mut png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
        png.extend(tail);
        prop_assert_eq!(media_type_of(&png), Some("image/png"));
    }
}
