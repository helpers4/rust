// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Extracts the token from an `Authorization: Bearer <token>` header value.
///
/// Follows RFC 7235 and RFC 6750: the scheme name is case-insensitive, it is followed by one or
/// more spaces, and the token is a non-empty `b64token` (letters, digits and `-` `.` `_` `~` `+`
/// `/`, optionally ending with `=` padding). Leading and trailing spaces or tabs around the whole
/// value are ignored. Anything else, including another scheme or an empty token, gives `None`.
///
/// Only the syntax is checked: whether the token is valid is up to the caller.
///
/// # Arguments
///
/// - `header` - The value of an `Authorization` header.
///
/// # Examples
///
/// ```
/// use helpers4::http::bearer_token;
///
/// assert_eq!(bearer_token("Bearer abc.def-123"), Some("abc.def-123"));
/// assert_eq!(bearer_token("bearer   abc"), Some("abc"));
/// assert_eq!(bearer_token("Basic dXNlcjpwYXNz"), None);
/// assert_eq!(bearer_token("Bearer "), None);
/// ```
#[must_use]
pub fn bearer_token(header: &str) -> Option<&str> {
    let header = header.trim_matches([' ', '\t']);
    let (scheme, rest) = header.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("Bearer") {
        return None;
    }
    let token = rest.trim_start_matches(' ');
    is_b64token(token).then_some(token)
}

/// `1*( ALPHA / DIGIT / "-" / "." / "_" / "~" / "+" / "/" ) *"="` (RFC 6750).
fn is_b64token(token: &str) -> bool {
    let body = token.trim_end_matches('=');
    !body.is_empty()
        && body.bytes().all(|b| {
            b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'+' | b'/')
        })
}

#[cfg(test)]
#[path = "bearer_token.test.rs"]
mod tests;

#[cfg(test)]
#[path = "bearer_token.spec.rs"]
mod spec;
