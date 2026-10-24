// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn v4_sets_the_version_and_variant_and_keeps_the_other_bits() {
    assert_eq!(
        Uuid::v4([0; 16]).to_string(),
        "00000000-0000-4000-8000-000000000000"
    );
    assert_eq!(
        Uuid::v4([0xff; 16]).to_string(),
        "ffffffff-ffff-4fff-bfff-ffffffffffff"
    );
    let id = Uuid::v4([0xab; 16]);
    assert_eq!(id.version(), 4);
    assert_eq!(id.as_bytes()[8] >> 6, 0b10); // RFC 9562 variant
}

#[test]
fn v7_puts_the_timestamp_first_and_sets_version_and_variant() {
    assert_eq!(
        Uuid::v7(0x0123_4567_89AB, [0xff; 10]).to_string(),
        "01234567-89ab-7fff-bfff-ffffffffffff"
    );
    assert_eq!(
        Uuid::v7(0, [0; 10]).to_string(),
        "00000000-0000-7000-8000-000000000000"
    );
    let id = Uuid::v7(1_700_000_000_000, [0x55; 10]);
    assert_eq!(id.version(), 7);
    assert_eq!(id.as_bytes()[8] >> 6, 0b10);
}

#[test]
fn v7_keeps_only_the_low_48_bits_of_the_timestamp() {
    assert_eq!(
        Uuid::v7(u64::MAX, [0; 10]).timestamp_millis(),
        Some(0xFFFF_FFFF_FFFF)
    );
    assert_eq!(Uuid::v7(1 << 48, [0; 10]).timestamp_millis(), Some(0));
}

#[test]
fn v7_ids_sort_by_timestamp() {
    let early = Uuid::v7(1_000, [0xff; 10]);
    let late = Uuid::v7(1_001, [0x00; 10]);
    assert!(early < late);
    assert!(early.to_string() < late.to_string());
}

#[test]
fn timestamp_is_only_defined_for_version_7() {
    assert_eq!(Uuid::v7(1234, [1; 10]).timestamp_millis(), Some(1234));
    assert_eq!(Uuid::v4([1; 16]).timestamp_millis(), None);
    assert_eq!(Uuid::NIL.timestamp_millis(), None);
}

#[test]
fn from_bytes_keeps_the_bytes_as_they_are() {
    let id = Uuid::from_bytes([7; 16]);
    assert_eq!(id.as_bytes(), &[7; 16]);
    assert_eq!(id.version(), 0);
}

#[test]
fn displays_lowercase_hyphenated_and_debugs_with_the_value() {
    let id = Uuid::from_bytes([0xAB; 16]);
    assert_eq!(id.to_string(), "abababab-abab-abab-abab-abababababab");
    assert_eq!(
        format!("{id:?}"),
        "Uuid(abababab-abab-abab-abab-abababababab)"
    );
    assert_eq!(
        Uuid::NIL.to_string(),
        "00000000-0000-0000-0000-000000000000"
    );
}

#[test]
fn parses_both_forms_in_any_case() {
    let expected = Uuid::from_bytes([
        0x12, 0x3e, 0x45, 0x67, 0xe8, 0x9b, 0x12, 0xd3, 0xa4, 0x56, 0x42, 0x66, 0x14, 0x17, 0x40,
        0x00,
    ]);
    assert_eq!(
        Uuid::parse("123e4567-e89b-12d3-a456-426614174000"),
        Ok(expected)
    );
    assert_eq!(
        Uuid::parse("123E4567-E89B-12D3-A456-426614174000"),
        Ok(expected)
    );
    assert_eq!(
        Uuid::parse("123e4567e89b12d3a456426614174000"),
        Ok(expected)
    );
    assert_eq!(
        "123e4567-e89b-12d3-a456-426614174000".parse::<Uuid>(),
        Ok(expected)
    );
    assert_eq!(expected.version(), 1);
}

#[test]
fn rejects_a_wrong_length() {
    assert_eq!(
        Uuid::parse(""),
        Err(ParseUuidError::InvalidLength { found: 0 })
    );
    assert_eq!(
        Uuid::parse("123e4567-e89b-12d3-a456-42661417400"),
        Err(ParseUuidError::InvalidLength { found: 35 })
    );
    assert_eq!(
        Uuid::parse("{123e4567-e89b-12d3-a456-426614174000}"),
        Err(ParseUuidError::InvalidLength { found: 38 })
    );
}

#[test]
fn rejects_a_bad_character_or_a_misplaced_hyphen_with_its_position() {
    assert_eq!(
        Uuid::parse("123e4567-e89b-12d3-a456-42661417400g"),
        Err(ParseUuidError::InvalidCharacter { index: 35 })
    );
    assert_eq!(
        Uuid::parse("123e4567_e89b-12d3-a456-426614174000"),
        Err(ParseUuidError::InvalidCharacter { index: 8 })
    );
    assert_eq!(
        Uuid::parse("123e45678-89b-12d3-a456-426614174000"),
        Err(ParseUuidError::InvalidCharacter { index: 8 })
    );
    assert_eq!(
        Uuid::parse("123e4567-e89b-12d3-a456-42661417-000"),
        Err(ParseUuidError::InvalidCharacter { index: 32 })
    );
    assert_eq!(
        Uuid::parse("123e4567e89b12d3a45642661417400-"),
        Err(ParseUuidError::InvalidCharacter { index: 31 })
    );
    assert_eq!(
        Uuid::parse("é23e4567e89b12d3a45642661417400"),
        Err(ParseUuidError::InvalidCharacter { index: 0 })
    );
}

#[test]
fn is_hashable_and_ordered() {
    let mut ids = std::collections::BTreeSet::new();
    ids.insert(Uuid::from_bytes([2; 16]));
    ids.insert(Uuid::from_bytes([1; 16]));
    assert_eq!(ids.first(), Some(&Uuid::from_bytes([1; 16])));
    let set: std::collections::HashSet<Uuid> = ids.iter().copied().collect();
    assert!(set.contains(&Uuid::from_bytes([2; 16])));
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod entropy {
    use super::*;

    #[test]
    fn new_v4_is_a_fresh_version_4_id() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        assert_eq!(a.map(|id| id.version()), Ok(4));
        assert_ne!(a, b);
    }

    #[test]
    fn new_v7_carries_the_current_time() {
        let before = crate::id::_unix_millis::unix_millis(std::time::SystemTime::now());
        let id = Uuid::new_v7();
        let after = crate::id::_unix_millis::unix_millis(std::time::SystemTime::now());
        assert_eq!(id.map(|id| id.version()), Ok(7));
        let stamp = id.ok().and_then(|id| id.timestamp_millis()).unwrap_or(0);
        assert!((before..=after).contains(&stamp));
        assert_ne!(id, Uuid::new_v7());
    }
}
