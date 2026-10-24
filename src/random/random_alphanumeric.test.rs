// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn has_the_requested_length_and_only_letters_and_digits() {
    for len in [0, 1, 10, 24, 200] {
        let token = random_alphanumeric(len).unwrap_or_default();
        assert_eq!(token.len(), len);
        assert!(token.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}

#[test]
fn two_draws_differ() {
    assert_ne!(random_alphanumeric(24), random_alphanumeric(24));
}

#[test]
fn maps_each_accepted_byte_to_its_character() {
    // 0 -> 'A', 25 -> 'Z', 26 -> 'a', 61 -> '9'; 62 and 63 wrap around to 'A' and 'B'; 247, the
    // largest accepted byte, is 3 × 62 + 61 -> '9'.
    let mut calls = 0;
    let mut fill = |buffer: &mut [u8]| {
        calls += 1;
        for (slot, byte) in buffer.iter_mut().zip([0u8, 25, 26, 61, 62, 63, 247]) {
            *slot = byte;
        }
        Ok(())
    };
    assert_eq!(alphanumeric_with(7, &mut fill), Ok("AZa9AB9".to_string()));
    assert_eq!(calls, 1);
}

#[test]
fn the_first_byte_that_would_favor_a_character_is_discarded() {
    // 248 = 4 x 62 is the first biased byte: it would map to 'A' once more than the others.
    let mut calls = 0;
    let mut fill = |buffer: &mut [u8]| {
        calls += 1;
        buffer.fill(if calls == 1 { 248 } else { 1 });
        Ok(())
    };
    assert_eq!(alphanumeric_with(3, &mut fill), Ok("BBB".to_string()));
    assert_eq!(calls, 2);
}

#[test]
fn discards_bytes_that_would_favor_some_characters() {
    // The first draw is all 248..=255 (rejected), the second is all zeros ('A').
    let mut calls = 0;
    let mut fill = |buffer: &mut [u8]| {
        calls += 1;
        buffer.fill(if calls == 1 { 255 } else { 0 });
        Ok(())
    };
    assert_eq!(alphanumeric_with(4, &mut fill), Ok("AAAA".to_string()));
    assert_eq!(calls, 2);
}

#[test]
fn does_not_ask_for_bytes_when_nothing_is_wanted() {
    let mut fill = |_: &mut [u8]| Err(EntropyError::from(getrandom::Error::UNSUPPORTED));
    assert_eq!(alphanumeric_with(0, &mut fill), Ok(String::new()));
}

#[test]
fn passes_the_error_through() {
    let error = EntropyError::from(getrandom::Error::UNSUPPORTED);
    let mut fill = |_: &mut [u8]| Err(error);
    assert_eq!(alphanumeric_with(5, &mut fill), Err(error));
}
