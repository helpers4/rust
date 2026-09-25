// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn empty_input_is_empty() {
    assert_eq!(base64_encode(b""), "");
}

#[test]
fn one_byte_gets_two_padding_characters() {
    assert_eq!(base64_encode(b"M"), "TQ==");
}

#[test]
fn two_bytes_get_one_padding_character() {
    assert_eq!(base64_encode(b"Ma"), "TWE=");
}

#[test]
fn three_bytes_need_no_padding() {
    assert_eq!(base64_encode(b"Man"), "TWFu");
}

#[test]
fn more_than_one_chunk() {
    assert_eq!(
        base64_encode(b"any carnal pleas"),
        "YW55IGNhcm5hbCBwbGVhcw=="
    );
}

#[test]
fn every_alphabet_character_round_trips_through_encode() {
    // 0..=63 as raw 6-bit values, packed 4 values (24 bits = 3 bytes) at a time so every
    // position of the alphabet is exercised without padding.
    let bytes: Vec<u8> = (0..252u8).collect();
    assert_eq!(base64_encode(&bytes).len(), 336);
}
