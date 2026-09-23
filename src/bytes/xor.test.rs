// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn xors_each_pair_of_bytes() {
    assert_eq!(
        xor(&[0b1100, 0b1010], &[0b1010, 0b0110]),
        Some(vec![0b0110, 0b1100])
    );
}

#[test]
fn different_lengths_give_none() {
    assert_eq!(xor(&[1], &[1, 2]), None);
    assert_eq!(xor(&[1, 2], &[1]), None);
}

#[test]
fn empty_slices_give_an_empty_result() {
    assert_eq!(xor(&[], &[]), Some(vec![]));
}
