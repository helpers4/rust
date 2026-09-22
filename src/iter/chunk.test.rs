// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn splits_into_even_chunks() {
    assert_eq!(chunk(1..=4, 2), vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn the_last_chunk_may_be_shorter() {
    assert_eq!(chunk(1..=5, 2), vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn a_size_of_one_gives_one_item_per_chunk() {
    assert_eq!(chunk(1..=3, 1), vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn a_size_at_least_as_large_as_the_input_gives_one_chunk() {
    assert_eq!(chunk(1..=3, 3), vec![vec![1, 2, 3]]);
    assert_eq!(chunk(1..=3, 10), vec![vec![1, 2, 3]]);
}

#[test]
fn empty_input_gives_no_chunks() {
    assert_eq!(chunk(Vec::<i32>::new(), 3), Vec::<Vec<i32>>::new());
}

#[test]
fn a_size_of_zero_gives_no_chunks() {
    assert_eq!(chunk(1..=3, 0), Vec::<Vec<i32>>::new());
    assert_eq!(chunk(Vec::<i32>::new(), 0), Vec::<Vec<i32>>::new());
}

#[test]
fn works_on_a_non_slice_iterator() {
    assert_eq!(
        chunk((1..10).filter(|n| n % 3 == 0), 2),
        vec![vec![3, 6], vec![9]]
    );
}
