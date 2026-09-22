// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn every_chunk_but_the_last_has_exactly_size_items(
        items in prop::collection::vec(0i32..100, 0..40),
        size in 1usize..8,
    ) {
        let chunks = chunk(items.clone(), size);
        for chunk in chunks.iter().rev().skip(1) {
            prop_assert_eq!(chunk.len(), size);
        }
        if let Some(last) = chunks.last() {
            prop_assert!(!last.is_empty() && last.len() <= size);
        }
    }

    #[test]
    fn flattening_the_chunks_gives_back_the_input(
        items in prop::collection::vec(0i32..100, 0..40),
        size in 1usize..8,
    ) {
        let flattened: Vec<i32> = chunk(items.clone(), size).into_iter().flatten().collect();
        prop_assert_eq!(flattened, items);
    }
}
