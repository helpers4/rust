// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn has_the_requested_length_and_only_alphabet_characters() {
    let alphabet = Alphabet::safe();
    for len in [0, 1, 21, 300] {
        let id = new_short_id(&alphabet, len).unwrap_or_default();
        assert_eq!(id.chars().count(), len);
        assert!(
            id.chars()
                .all(|c| (0..alphabet.size()).any(|i| alphabet.pick(i) == c))
        );
    }
}

#[test]
fn two_ids_differ() {
    assert_ne!(
        new_short_id(&Alphabet::url_safe(), 21),
        new_short_id(&Alphabet::url_safe(), 21)
    );
}

#[test]
fn maps_bytes_to_characters_by_position_and_discards_the_biased_zone() {
    // Alphabet of 10: the limit is 250. 0 -> 0, 9 -> 9, 10 -> 0, 249 -> 9, then 7 -> 7;
    // 250..=255 are discarded.
    let mut fill = |buffer: &mut [u8]| {
        for (slot, byte) in buffer.iter_mut().zip([0u8, 9, 10, 249, 250, 255, 7]) {
            *slot = byte;
        }
        Ok(())
    };
    assert_eq!(
        short_id_with(&Alphabet::numbers(), 5, &mut fill),
        Ok("09097".to_string())
    );
}

#[test]
fn draws_again_when_a_whole_buffer_was_discarded() {
    let mut calls = 0;
    let mut fill = |buffer: &mut [u8]| {
        calls += 1;
        buffer.fill(if calls == 1 { 255 } else { 3 });
        Ok(())
    };
    assert_eq!(
        short_id_with(&Alphabet::numbers(), 3, &mut fill),
        Ok("333".to_string())
    );
    assert_eq!(calls, 2);
}

#[test]
fn accepts_every_byte_for_a_256_character_alphabet() {
    let all: String = (0..256u32).filter_map(char::from_u32).collect();
    let alphabet = Alphabet::new(&all).unwrap_or_else(|_| Alphabet::numbers());
    let mut fill = |buffer: &mut [u8]| {
        buffer.fill(255);
        Ok(())
    };
    assert_eq!(
        short_id_with(&alphabet, 2, &mut fill),
        Ok("\u{ff}\u{ff}".to_string())
    );
}

#[test]
fn does_not_ask_for_bytes_when_nothing_is_wanted_and_passes_errors_through() {
    let error = EntropyError::from(getrandom::Error::UNSUPPORTED);
    let mut failing = |_: &mut [u8]| Err(error);
    assert_eq!(
        short_id_with(&Alphabet::numbers(), 0, &mut failing),
        Ok(String::new())
    );
    assert_eq!(
        short_id_with(&Alphabet::numbers(), 4, &mut failing),
        Err(error)
    );
}
