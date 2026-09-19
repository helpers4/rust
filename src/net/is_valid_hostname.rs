// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::error::HostnameError;

/// Checks that `s` is a valid hostname (RFC 1035 and RFC 1123, ASCII only).
///
/// The rules: at most 253 octets, not counting one optional trailing dot; labels of 1 to 63
/// octets made of ASCII letters, digits and hyphens; no label starts or ends with a hyphen. A
/// label may start with a digit. Underscores are refused (they are not hostname characters), and
/// so are non-ASCII characters: convert an internationalized name to its `xn--` form first.
///
/// # Errors
///
/// A [`HostnameError`] naming the first rule that fails.
///
/// # Examples
///
/// ```
/// use helpers4::net::{is_valid_hostname, HostnameError};
///
/// assert!(is_valid_hostname("example.com").is_ok());
/// assert!(is_valid_hostname("localhost.").is_ok());
/// assert_eq!(is_valid_hostname("-bad.example"), Err(HostnameError::HyphenEdge));
/// assert_eq!(is_valid_hostname("a..b"), Err(HostnameError::EmptyLabel));
/// ```
pub fn is_valid_hostname(s: &str) -> Result<(), HostnameError> {
    if s.is_empty() {
        return Err(HostnameError::Empty);
    }
    let name = s.strip_suffix('.').unwrap_or(s);
    if name.len() > 253 {
        return Err(HostnameError::TooLong);
    }
    let mut offset = 0;
    for label in name.split('.') {
        check_label(label, offset)?;
        offset += label.len() + 1;
    }
    Ok(())
}

fn check_label(label: &str, offset: usize) -> Result<(), HostnameError> {
    if label.is_empty() {
        return Err(HostnameError::EmptyLabel);
    }
    if label.len() > 63 {
        return Err(HostnameError::LabelTooLong);
    }
    if let Some((index, found)) = label
        .char_indices()
        .find(|&(_, c)| !(c.is_ascii_alphanumeric() || c == '-'))
    {
        return Err(HostnameError::InvalidChar {
            index: offset + index,
            found,
        });
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(HostnameError::HyphenEdge);
    }
    Ok(())
}

#[cfg(test)]
#[path = "is_valid_hostname.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_valid_hostname.spec.rs"]
mod spec;
