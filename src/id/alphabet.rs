// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;

use super::AlphabetError;

/// The characters a short id is made of: between 2 and 256 distinct characters.
///
/// Presets cover the common cases; [`new`](Self::new) checks a custom set. Every character has
/// the same probability in an id, which needs them distinct (a repeated character would be more
/// likely) and no more than a byte of randomness can index (256).
///
/// # Examples
///
/// ```
/// use helpers4::id::Alphabet;
///
/// assert_eq!(Alphabet::numbers().size(), 10);
/// assert_eq!(Alphabet::safe().size(), 56); // no 0 O o 1 I l: fine to read aloud or retype
/// assert_eq!(Alphabet::new("ACGT")?.size(), 4);
/// # Ok::<(), helpers4::id::AlphabetError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alphabet {
    chars: Vec<char>,
}

impl Alphabet {
    /// A custom alphabet.
    ///
    /// # Arguments
    ///
    /// - `characters` - The characters, each once, in any order.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Errors
    ///
    /// Returns an [`AlphabetError`] for fewer than 2 characters, more than 256, or a repeated
    /// character.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::{Alphabet, AlphabetError};
    ///
    /// assert!(Alphabet::new("01").is_ok());
    /// assert_eq!(Alphabet::new("aa"), Err(AlphabetError::Duplicate { character: 'a' }));
    /// ```
    pub fn new(characters: &str) -> Result<Self, AlphabetError> {
        let chars: Vec<char> = characters.chars().collect();
        if chars.len() < 2 {
            return Err(AlphabetError::TooShort { found: chars.len() });
        }
        if chars.len() > 256 {
            return Err(AlphabetError::TooLong { found: chars.len() });
        }
        let mut seen = HashSet::new();
        for &character in &chars {
            if !seen.insert(character) {
                return Err(AlphabetError::Duplicate { character });
            }
        }
        Ok(Self { chars })
    }

    /// The digits `0-9`.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::numbers().size(), 10);
    /// ```
    #[must_use]
    pub fn numbers() -> Self {
        Self::preset("0123456789")
    }

    /// The lowercase letters `a-z`.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::lowercase().size(), 26);
    /// ```
    #[must_use]
    pub fn lowercase() -> Self {
        Self::preset("abcdefghijklmnopqrstuvwxyz")
    }

    /// The uppercase letters `A-Z`.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::uppercase().size(), 26);
    /// ```
    #[must_use]
    pub fn uppercase() -> Self {
        Self::preset("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }

    /// Letters and digits, `A-Za-z0-9`.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::alphanumeric().size(), 62);
    /// ```
    #[must_use]
    pub fn alphanumeric() -> Self {
        Self::preset("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    }

    /// The 64 characters that are safe in a URL path and a file name without escaping:
    /// `A-Za-z0-9_-`.
    ///
    /// # Returns
    ///
    /// The alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::url_safe().size(), 64);
    /// ```
    #[must_use]
    pub fn url_safe() -> Self {
        Self::preset("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-")
    }

    /// Letters and digits without the characters that look alike, for ids a person reads or
    /// types: no `0`, `1`, `O`, `o`, `I` or `l`.
    ///
    /// # Returns
    ///
    /// The alphabet, 56 characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::safe().size(), 56);
    /// ```
    #[must_use]
    pub fn safe() -> Self {
        Self::preset("23456789abcdefghijkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ")
    }

    /// The number of characters.
    ///
    /// # Returns
    ///
    /// A number from 2 to 256.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Alphabet;
    ///
    /// assert_eq!(Alphabet::new("abc")?.size(), 3);
    /// # Ok::<(), helpers4::id::AlphabetError>(())
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        self.chars.len()
    }

    /// The character at `index`, or the first one for an index past the end (callers pass an
    /// index below [`size`](Self::size)).
    pub(crate) fn pick(&self, index: usize) -> char {
        self.chars
            .get(index)
            .or_else(|| self.chars.first())
            .copied()
            .unwrap_or('\0')
    }

    fn preset(characters: &str) -> Self {
        Self {
            chars: characters.chars().collect(),
        }
    }
}

#[cfg(test)]
#[path = "alphabet.test.rs"]
mod tests;
