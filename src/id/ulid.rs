// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;
use std::str::FromStr;

use super::ParseUlidError;

const SYMBOLS: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// A ULID: 128 bits made of a 48-bit millisecond timestamp and 80 random bits, written as 26
/// characters of Crockford's base32 (`01ARYZ6S41TSV4RRFFQ69G5FAV`).
///
/// Ulids sort by creation time, both as numbers and as text, and are shorter than a UUID. Time
/// and randomness are **parameters** of [`from_parts`](Self::from_parts); with the
/// `random-entropy` feature, `new` takes them from the system. Ids made in the
/// same millisecond are not ordered among themselves (there is no monotonic counter).
///
/// # Examples
///
/// ```
/// use helpers4::id::Ulid;
///
/// let id = Ulid::from_parts(1_469_918_176_385, [0; 10]);
/// assert_eq!(id.to_string(), "01ARYZ6S410000000000000000");
/// assert_eq!(id.timestamp_millis(), 1_469_918_176_385);
/// assert_eq!(Ulid::parse("01ARYZ6S41TSV4RRFFQ69G5FAV").map(|u| u.timestamp_millis()), Ok(1_469_918_176_385));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ulid(u128);

impl Ulid {
    /// Builds a ULID from a timestamp and 80 random bits. Only the low 48 bits of
    /// `unix_millis` are kept, enough until the year 10889.
    ///
    /// # Arguments
    ///
    /// - `unix_millis` - Milliseconds since 1970-01-01 UTC.
    /// - `random` - 10 random bytes, from a secure source for an id that must be unpredictable.
    ///
    /// # Returns
    ///
    /// The ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// assert_eq!(Ulid::from_parts(0, [0xff; 10]).to_string(), "0000000000ZZZZZZZZZZZZZZZZ");
    /// ```
    #[must_use]
    pub const fn from_parts(unix_millis: u64, random: [u8; 10]) -> Self {
        let m = unix_millis.to_be_bytes();
        Self(u128::from_be_bytes([
            m[2], m[3], m[4], m[5], m[6], m[7], random[0], random[1], random[2], random[3],
            random[4], random[5], random[6], random[7], random[8], random[9],
        ]))
    }

    /// Wraps a 128-bit value as it is.
    ///
    /// # Arguments
    ///
    /// - `value` - The 128 bits: the timestamp in the top 48, then the random part.
    ///
    /// # Returns
    ///
    /// The ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// assert_eq!(Ulid::from_u128(1).to_string(), "00000000000000000000000001");
    /// ```
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    /// The ULID as a 128-bit number.
    ///
    /// # Returns
    ///
    /// The timestamp in the top 48 bits, then the random part.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// assert_eq!(Ulid::from_parts(0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 7]).as_u128(), 7);
    /// ```
    #[must_use]
    pub const fn as_u128(&self) -> u128 {
        self.0
    }

    /// The creation time stored in the ULID.
    ///
    /// # Returns
    ///
    /// Milliseconds since 1970-01-01 UTC.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// assert_eq!(Ulid::from_parts(1234, [9; 10]).timestamp_millis(), 1234);
    /// ```
    #[must_use]
    pub const fn timestamp_millis(&self) -> u64 {
        let b = self.0.to_be_bytes();
        u64::from_be_bytes([0, 0, b[0], b[1], b[2], b[3], b[4], b[5]])
    }

    /// Reads a ULID from its 26-character form, in upper or lower case. Only the 32 symbols of
    /// Crockford's base32 are accepted (`I`, `L`, `O` and `U` are not).
    ///
    /// # Arguments
    ///
    /// - `text` - The text to read.
    ///
    /// # Returns
    ///
    /// The ULID.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseUlidError`] for a length other than 26, a character outside the
    /// alphabet, or a first character above `7` (the value would not fit in 128 bits).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// let id = Ulid::parse("01aryz6s41tsv4rrffq69g5fav")?;
    /// assert_eq!(id.to_string(), "01ARYZ6S41TSV4RRFFQ69G5FAV");
    /// # Ok::<(), helpers4::id::ParseUlidError>(())
    /// ```
    pub fn parse(text: &str) -> Result<Self, ParseUlidError> {
        let bytes = text.as_bytes();
        if bytes.len() != 26 {
            return Err(ParseUlidError::InvalidLength { found: bytes.len() });
        }
        let mut value: u128 = 0;
        for (index, &byte) in bytes.iter().enumerate() {
            let upper = byte.to_ascii_uppercase();
            let digit = SYMBOLS
                .iter()
                .position(|&symbol| symbol == upper)
                .and_then(|position| u8::try_from(position).ok())
                .ok_or(ParseUlidError::InvalidCharacter { index })?;
            if index == 0 && digit > 7 {
                return Err(ParseUlidError::Overflow);
            }
            value = (value << 5) + u128::from(digit);
        }
        Ok(Self(value))
    }
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
impl Ulid {
    /// A ULID with the current time from the system clock and random bits from the operating
    /// system.
    ///
    /// Needs the `random-entropy` feature. A clock set before 1970 gives a timestamp of `0`.
    ///
    /// # Returns
    ///
    /// The ULID.
    ///
    /// # Errors
    ///
    /// Returns an [`EntropyError`](crate::random::EntropyError) when the operating system cannot
    /// provide random bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Ulid;
    ///
    /// let id = Ulid::new()?;
    /// assert_eq!(id.to_string().len(), 26);
    /// # Ok::<(), helpers4::random::EntropyError>(())
    /// ```
    pub fn new() -> Result<Self, crate::random::EntropyError> {
        let mut random = [0u8; 10];
        crate::random::_fill_os::fill_os(&mut random).map(|()| {
            Self::from_parts(
                super::_unix_millis::unix_millis(std::time::SystemTime::now()),
                random,
            )
        })
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::with_capacity(26);
        for index in 0..26u32 {
            let shift = 5 * (25 - index);
            let symbol = usize::try_from((self.0 >> shift) & 31).unwrap_or(0);
            text.push(char::from(SYMBOLS[symbol]));
        }
        f.write_str(&text)
    }
}

impl fmt::Debug for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ulid({self})")
    }
}

impl FromStr for Ulid {
    type Err = ParseUlidError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "ulid.test.rs"]
mod tests;

#[cfg(test)]
#[path = "ulid.spec.rs"]
mod spec;
