// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn from_parts_writes_the_timestamp_then_the_random_part() {
    assert_eq!(
        Ulid::from_parts(1_469_918_176_385, [0; 10]).to_string(),
        "01ARYZ6S410000000000000000"
    );
    assert_eq!(
        Ulid::from_parts(0, [0xff; 10]).to_string(),
        "0000000000ZZZZZZZZZZZZZZZZ"
    );
    assert_eq!(
        Ulid::from_parts(0, [0; 10]).to_string(),
        "00000000000000000000000000"
    );
}

#[test]
fn keeps_only_the_low_48_bits_of_the_timestamp() {
    assert_eq!(
        Ulid::from_parts(u64::MAX, [0; 10]).timestamp_millis(),
        0xFFFF_FFFF_FFFF
    );
    assert_eq!(
        Ulid::from_parts(0xFFFF_FFFF_FFFF, [0xff; 10]).to_string(),
        "7ZZZZZZZZZZZZZZZZZZZZZZZZZ"
    );
}

#[test]
fn reads_the_specification_example() {
    let id = Ulid::parse("01ARYZ6S41TSV4RRFFQ69G5FAV");
    assert_eq!(id.map(|u| u.timestamp_millis()), Ok(1_469_918_176_385));
    assert_eq!(
        id.map(|u| u.to_string()),
        Ok("01ARYZ6S41TSV4RRFFQ69G5FAV".to_string())
    );
    assert_eq!("01aryz6s41tsv4rrffq69g5fav".parse::<Ulid>(), id);
}

#[test]
fn converts_to_and_from_a_number() {
    assert_eq!(Ulid::from_u128(7).as_u128(), 7);
    assert_eq!(Ulid::from_u128(u128::MAX).as_u128(), u128::MAX);
    assert_eq!(
        Ulid::from_u128(u128::MAX).to_string(),
        "7ZZZZZZZZZZZZZZZZZZZZZZZZZ"
    );
}

#[test]
fn sorts_by_time_as_numbers_and_as_text() {
    let early = Ulid::from_parts(1_000, [0xff; 10]);
    let late = Ulid::from_parts(1_001, [0x00; 10]);
    assert!(early < late);
    assert!(early.to_string() < late.to_string());
}

#[test]
fn debugs_with_the_value() {
    assert_eq!(
        format!("{:?}", Ulid::from_u128(1)),
        "Ulid(00000000000000000000000001)"
    );
}

#[test]
fn rejects_a_wrong_length() {
    assert_eq!(
        Ulid::parse(""),
        Err(ParseUlidError::InvalidLength { found: 0 })
    );
    assert_eq!(
        Ulid::parse("01ARYZ6S41TSV4RRFFQ69G5FA"),
        Err(ParseUlidError::InvalidLength { found: 25 })
    );
    assert_eq!(
        Ulid::parse("01ARYZ6S41TSV4RRFFQ69G5FAVV"),
        Err(ParseUlidError::InvalidLength { found: 27 })
    );
}

#[test]
fn rejects_characters_outside_crockford_base32_with_their_position() {
    for (text, index) in [
        ("01ARYZ6S41TSV4RRFFQ69G5FAI", 25), // I, L, O and U are not symbols
        ("01ARYZ6S41TSV4RRFFQ69G5FAL", 25),
        ("01ARYZ6S41TSV4RRFFQ69G5FAO", 25),
        ("01ARYZ6S41TSV4RRFFQ69G5FAU", 25),
        ("01ARYZ6S41TSV4RRFFQ69G5FA-", 25),
        ("0!ARYZ6S41TSV4RRFFQ69G5FAV", 1),
    ] {
        assert_eq!(
            Ulid::parse(text),
            Err(ParseUlidError::InvalidCharacter { index }),
            "{text}"
        );
    }
}

#[test]
fn rejects_a_first_character_above_seven() {
    assert_eq!(
        Ulid::parse("80000000000000000000000000"),
        Err(ParseUlidError::Overflow)
    );
    assert_eq!(
        Ulid::parse("ZZZZZZZZZZZZZZZZZZZZZZZZZZ"),
        Err(ParseUlidError::Overflow)
    );
    assert!(Ulid::parse("70000000000000000000000000").is_ok());
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod entropy {
    use super::*;

    #[test]
    fn new_carries_the_current_time_and_differs_each_time() {
        let before = crate::id::_unix_millis::unix_millis(std::time::SystemTime::now());
        let id = Ulid::new();
        let after = crate::id::_unix_millis::unix_millis(std::time::SystemTime::now());
        let stamp = id.map_or(0, |id| id.timestamp_millis());
        assert!((before..=after).contains(&stamp));
        assert_ne!(id, Ulid::new());
    }
}
