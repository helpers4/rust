// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_between_2_and_256_distinct_characters() {
    assert_eq!(Alphabet::new("ab").map(|a| a.size()), Ok(2));
    let all: String = (0..256u32).filter_map(char::from_u32).collect();
    assert_eq!(Alphabet::new(&all).map(|a| a.size()), Ok(256));
    assert_eq!(Alphabet::new("日本語").map(|a| a.size()), Ok(3));
}

#[test]
fn rejects_too_few_too_many_and_repeated_characters() {
    assert_eq!(Alphabet::new(""), Err(AlphabetError::TooShort { found: 0 }));
    assert_eq!(
        Alphabet::new("a"),
        Err(AlphabetError::TooShort { found: 1 })
    );
    let too_many: String = (0..257u32).filter_map(char::from_u32).collect();
    assert_eq!(
        Alphabet::new(&too_many),
        Err(AlphabetError::TooLong { found: 257 })
    );
    assert_eq!(
        Alphabet::new("abcb"),
        Err(AlphabetError::Duplicate { character: 'b' })
    );
}

#[test]
fn presets_have_the_documented_sizes_and_distinct_characters() {
    let presets = [
        (Alphabet::numbers(), 10),
        (Alphabet::lowercase(), 26),
        (Alphabet::uppercase(), 26),
        (Alphabet::alphanumeric(), 62),
        (Alphabet::url_safe(), 64),
        (Alphabet::safe(), 56),
    ];
    for (alphabet, size) in presets {
        assert_eq!(alphabet.size(), size);
        let text: String = (0..size).map(|i| alphabet.pick(i)).collect();
        assert_eq!(Alphabet::new(&text), Ok(alphabet));
    }
}

#[test]
fn safe_leaves_out_the_characters_that_look_alike() {
    let safe: String = (0..Alphabet::safe().size())
        .map(|i| Alphabet::safe().pick(i))
        .collect();
    for ambiguous in ['0', '1', 'O', 'o', 'I', 'l'] {
        assert!(!safe.contains(ambiguous), "{ambiguous}");
    }
}

#[test]
fn url_safe_only_has_url_safe_characters() {
    let text: String = (0..64).map(|i| Alphabet::url_safe().pick(i)).collect();
    assert!(
        text.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    );
}

#[test]
fn pick_returns_the_character_at_an_index_and_falls_back_past_the_end() {
    let alphabet = Alphabet::new("xyz").unwrap_or_else(|_| Alphabet::numbers());
    assert_eq!(alphabet.pick(0), 'x');
    assert_eq!(alphabet.pick(2), 'z');
    assert_eq!(alphabet.pick(99), 'x');
}
