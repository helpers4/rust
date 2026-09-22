// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::error::HostnameError;

/// Checks that `hostname` is a valid hostname (RFC 1035 and RFC 1123, ASCII only).
///
/// The rules: at most 253 octets, not counting one optional trailing dot; labels of 1 to 63
/// octets made of ASCII letters, digits and hyphens; no label starts or ends with a hyphen. A
/// label may start with a digit, but the **last** label may not be a number (decimal like `1`, or
/// hexadecimal like `0x7f`): RFC 1123 section 2.1 keeps the top-level label alphabetic so that a
/// hostname is never mistaken for an address, and URL parsers do read `127.1`, `2130706433` or
/// `0x7f000001` as `127.0.0.1`. Underscores are refused (they are not hostname characters), and so
/// are non-ASCII characters: convert an internationalized name to its `xn--` form first.
///
/// This checks syntax only. It is not an SSRF check: a name that passes can still resolve to a
/// private address. To guard a server that fetches user-supplied URLs, parse the URL first, then
/// check the address you will actually connect to with
/// [`is_public_ip`](super::is_public_ip).
///
/// # Arguments
///
/// - `hostname` - The hostname to validate.
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
/// assert_eq!(is_valid_hostname("127.0.0.1"), Err(HostnameError::NumericLastLabel));
/// ```
pub fn is_valid_hostname(hostname: &str) -> Result<(), HostnameError> {
    if hostname.is_empty() {
        return Err(HostnameError::Empty);
    }
    let name = hostname.strip_suffix('.').unwrap_or(hostname);
    if name.len() > 253 {
        return Err(HostnameError::TooLong);
    }
    let mut offset = 0;
    for label in name.split('.') {
        check_label(label, offset)?;
        offset += label.len() + 1;
    }
    if is_number(name.rsplit('.').next().unwrap_or(name)) {
        return Err(HostnameError::NumericLastLabel);
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

/// A non-empty label of decimal digits, or `0x` / `0X` followed by hexadecimal digits (possibly
/// none): what a URL parser's "ends in a number" check treats as an IPv4 number.
fn is_number(label: &str) -> bool {
    label.bytes().all(|b| b.is_ascii_digit())
        || label
            .strip_prefix("0x")
            .or_else(|| label.strip_prefix("0X"))
            .is_some_and(|rest| rest.bytes().all(|b| b.is_ascii_hexdigit()))
}

#[cfg(test)]
#[path = "is_valid_hostname.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_valid_hostname.spec.rs"]
mod spec;
