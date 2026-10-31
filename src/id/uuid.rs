// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;
use std::str::FromStr;

use super::ParseUuidError;

/// A UUID (RFC 9562): 128 bits, written `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`.
///
/// Two kinds can be built here, from randomness and time **you pass in** (see
/// `new_v4` and `new_v7`, with the `random-entropy` feature, to
/// take them from the system):
///
/// - [`v4`](Self::v4): 122 random bits, the general-purpose unique id;
/// - [`v7`](Self::v7): a millisecond timestamp followed by random bits, so ids sort by creation
///   time, which keeps database indexes compact.
///
/// `Display` writes the lowercase hyphenated form; [`parse`](Self::parse) reads it back (and the
/// 32-digit form without hyphens, in any case).
///
/// # Examples
///
/// ```
/// use helpers4::id::Uuid;
///
/// let id = Uuid::v4([0xff; 16]); // pass random bytes from your entropy source
/// assert_eq!(id.to_string(), "ffffffff-ffff-4fff-bfff-ffffffffffff");
/// assert_eq!(id.version(), 4);
///
/// let sortable = Uuid::v7(1_700_000_000_000, [0; 10]);
/// assert_eq!(sortable.timestamp_millis(), Some(1_700_000_000_000));
/// assert_eq!(Uuid::parse(&sortable.to_string()), Ok(sortable));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// The nil UUID, all zeros.
    pub const NIL: Self = Self([0; 16]);

    /// Wraps 16 bytes as they are, without touching the version or variant bits.
    ///
    /// # Arguments
    ///
    /// - `bytes` - The 16 bytes of the UUID.
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// let id = Uuid::from_bytes([1; 16]);
    /// assert_eq!(id.as_bytes(), &[1; 16]);
    /// ```
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// The 16 bytes of the UUID.
    ///
    /// # Returns
    ///
    /// A reference to the bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// assert_eq!(Uuid::NIL.as_bytes(), &[0; 16]);
    /// ```
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Builds a version 4 UUID from 16 random bytes: 6 bits are overwritten by the version and
    /// variant, leaving 122 random bits.
    ///
    /// # Arguments
    ///
    /// - `random` - Random bytes, from a secure source for an id that must be unpredictable
    ///   (see `random_bytes`).
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// assert_eq!(Uuid::v4([0; 16]).to_string(), "00000000-0000-4000-8000-000000000000");
    /// ```
    #[must_use]
    pub const fn v4(random: [u8; 16]) -> Self {
        let mut bytes = random;
        bytes[6] = (bytes[6] & 0x0f) + 0x40;
        bytes[8] = (bytes[8] & 0x3f) + 0x80;
        Self(bytes)
    }

    /// Builds a version 7 UUID: a 48-bit millisecond Unix timestamp, then 74 random bits.
    ///
    /// Ids made later sort after earlier ones (to the millisecond; within one millisecond the
    /// random bits decide). Only the low 48 bits of `unix_millis` are kept, enough until the year
    /// 10889.
    ///
    /// # Arguments
    ///
    /// - `unix_millis` - Milliseconds since 1970-01-01 UTC.
    /// - `random` - 10 random bytes (74 bits are used).
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// let id = Uuid::v7(0x0123_4567_89AB, [0xff; 10]);
    /// assert_eq!(id.to_string(), "01234567-89ab-7fff-bfff-ffffffffffff");
    /// ```
    #[must_use]
    pub const fn v7(unix_millis: u64, random: [u8; 10]) -> Self {
        let m = unix_millis.to_be_bytes();
        Self([
            m[2],
            m[3],
            m[4],
            m[5],
            m[6],
            m[7],
            0x70 + (random[0] & 0x0f),
            random[1],
            0x80 + (random[2] & 0x3f),
            random[3],
            random[4],
            random[5],
            random[6],
            random[7],
            random[8],
            random[9],
        ])
    }

    /// The version number in the id (`4` for [`v4`](Self::v4), `7` for [`v7`](Self::v7)).
    ///
    /// # Returns
    ///
    /// The high nibble of the seventh byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// assert_eq!(Uuid::v7(0, [0; 10]).version(), 7);
    /// assert_eq!(Uuid::NIL.version(), 0);
    /// ```
    #[must_use]
    pub const fn version(&self) -> u8 {
        self.0[6] >> 4
    }

    /// The creation time stored in a version 7 UUID.
    ///
    /// # Returns
    ///
    /// Milliseconds since 1970-01-01 UTC, or `None` for any other version.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// assert_eq!(Uuid::v7(1234, [0; 10]).timestamp_millis(), Some(1234));
    /// assert_eq!(Uuid::v4([0; 16]).timestamp_millis(), None);
    /// ```
    #[must_use]
    pub const fn timestamp_millis(&self) -> Option<u64> {
        if self.version() == 7 {
            let b = &self.0;
            Some(u64::from_be_bytes([
                0, 0, b[0], b[1], b[2], b[3], b[4], b[5],
            ]))
        } else {
            None
        }
    }

    /// Reads a UUID from text: the hyphenated form `8-4-4-4-12` or 32 digits without hyphens,
    /// upper or lower case. Braces and `urn:uuid:` prefixes are not accepted.
    ///
    /// The version and variant bits are not checked: any 128-bit value is a valid UUID here.
    ///
    /// # Arguments
    ///
    /// - `text` - The text to read.
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseUuidError`] for a length other than 36 or 32, a character that is not a
    /// hexadecimal digit, or a hyphen that is not at its place.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// let id = Uuid::parse("123E4567-e89b-12d3-a456-426614174000")?;
    /// assert_eq!(id.version(), 1);
    /// assert_eq!(id.to_string(), "123e4567-e89b-12d3-a456-426614174000");
    /// assert_eq!(Uuid::parse("123e4567e89b12d3a456426614174000"), Ok(id));
    /// # Ok::<(), helpers4::id::ParseUuidError>(())
    /// ```
    pub fn parse(text: &str) -> Result<Self, ParseUuidError> {
        let bytes = text.as_bytes();
        let hyphenated = match bytes.len() {
            36 => true,
            32 => false,
            found => return Err(ParseUuidError::InvalidLength { found }),
        };
        let mut out = [0u8; 16];
        let mut digits = 0usize;
        for (index, &byte) in bytes.iter().enumerate() {
            if hyphenated && matches!(index, 8 | 13 | 18 | 23) {
                if byte != b'-' {
                    return Err(ParseUuidError::InvalidCharacter { index });
                }
                continue;
            }
            let digit = hex_value(byte).ok_or(ParseUuidError::InvalidCharacter { index })?;
            if digits % 2 == 0 {
                out[digits / 2] = digit << 4;
            } else {
                out[digits / 2] |= digit;
            }
            digits += 1;
        }
        Ok(Self(out))
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
impl Uuid {
    /// A random (version 4) UUID, with bytes from the operating system.
    ///
    /// Needs the `random-entropy` feature.
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`EntropyError`](crate::random::EntropyError) when the operating system cannot
    /// provide random bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// let id = Uuid::new_v4()?;
    /// assert_eq!(id.version(), 4);
    /// # Ok::<(), helpers4::random::EntropyError>(())
    /// ```
    pub fn new_v4() -> Result<Self, crate::random::EntropyError> {
        let mut random = [0u8; 16];
        crate::random::_fill_os::fill_os(&mut random).map(|()| Self::v4(random))
    }

    /// A time-sortable (version 7) UUID, with the current time from the system clock and
    /// random bytes from the operating system.
    ///
    /// Needs the `random-entropy` feature. A clock set before 1970 gives a timestamp of `0`.
    ///
    /// # Returns
    ///
    /// The UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`EntropyError`](crate::random::EntropyError) when the operating system cannot
    /// provide random bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::id::Uuid;
    ///
    /// let id = Uuid::new_v7()?;
    /// assert_eq!(id.version(), 7);
    /// assert!(id.timestamp_millis().is_some_and(|ms| ms > 1_700_000_000_000));
    /// # Ok::<(), helpers4::random::EntropyError>(())
    /// ```
    pub fn new_v7() -> Result<Self, crate::random::EntropyError> {
        let mut random = [0u8; 10];
        crate::random::_fill_os::fill_os(&mut random).map(|()| {
            Self::v7(
                super::_unix_millis::unix_millis(std::time::SystemTime::now()),
                random,
            )
        })
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut text = String::with_capacity(36);
        for (index, byte) in self.0.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                text.push('-');
            }
            text.push(char::from(DIGITS[usize::from(byte >> 4)]));
            text.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
        }
        f.write_str(&text)
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uuid({self})")
    }
}

impl FromStr for Uuid {
    type Err = ParseUuidError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

#[cfg(test)]
#[path = "uuid.test.rs"]
mod tests;

#[cfg(test)]
#[path = "uuid.spec.rs"]
mod spec;
