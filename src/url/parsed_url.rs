// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::UrlError;
use std::fmt;
use std::str::FromStr;

/// A parsed URL: `scheme://userinfo@host:port/path?query#fragment`.
///
/// The parts are kept as written (not percent-decoded), except that the scheme and the host are
/// lowercased, which URLs treat as case-insensitive. Any part but the scheme and the path may be
/// absent. `Display` writes the URL back, so a normalized URL round-trips. This is a general
/// RFC 3986 parser, not the WHATWG one: it does not know special schemes, punycode, or percent-
/// encode for you (see [`percent_encode`](super::percent_encode)), and it rejects spaces and
/// control characters instead of fixing them.
///
/// # Examples
///
/// ```
/// use helpers4::url::Url;
///
/// let url = Url::parse("https://Example.com:8443/a/b?x=1#top")?;
/// assert_eq!(url.scheme(), "https");
/// assert_eq!(url.host(), Some("example.com"));
/// assert_eq!(url.port(), Some(8443));
/// assert_eq!(url.path(), "/a/b");
/// assert_eq!(url.query(), Some("x=1"));
/// assert_eq!(url.fragment(), Some("top"));
/// assert_eq!(url.join("../c")?.to_string(), "https://example.com:8443/c");
/// # Ok::<(), helpers4::url::UrlError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    scheme: String,
    userinfo: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

/// A URL reference cut into its raw pieces.
#[derive(Clone, Copy)]
struct Pieces<'a> {
    authority: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

impl Url {
    /// Parses an absolute URL.
    ///
    /// # Arguments
    ///
    /// - `input` - The URL, which must start with a scheme (`https:`, `mailto:`, `file:`, ...).
    ///
    /// # Errors
    ///
    /// A [`UrlError`] for a space or control character, a missing or invalid scheme, a malformed
    /// host, or a port that is not a number from 0 to 65535.
    pub fn parse(input: &str) -> Result<Self, UrlError> {
        check_characters(input)?;
        let (scheme, rest) = split_scheme(input)?;
        Self::from_pieces(scheme, split_pieces(rest))
    }

    /// Resolves `reference` against this URL, as a browser resolves a link on a page (RFC 3986,
    /// section 5).
    ///
    /// The reference may be absolute (`https://other.example/`), start with `//` (keep the
    /// scheme), a path (`/a`, `b`, `../c`), only a query (`?x=1`) or only a fragment (`#top`).
    /// `.` and `..` segments are resolved.
    ///
    /// # Arguments
    ///
    /// - `reference` - The URL reference to resolve.
    ///
    /// # Errors
    ///
    /// The same [`UrlError`]s as [`Url::parse`], for the reference.
    pub fn join(&self, reference: &str) -> Result<Self, UrlError> {
        check_characters(reference)?;
        if has_scheme(reference) {
            return Self::parse(reference);
        }
        let target = split_pieces(reference);
        if target.authority.is_some() {
            return Self::from_pieces(self.scheme.clone(), target);
        }
        let (path, query) = if target.path.is_empty() {
            (
                self.path.clone(),
                target
                    .query
                    .map(str::to_string)
                    .or_else(|| self.query.clone()),
            )
        } else if target.path.starts_with('/') {
            (
                remove_dot_segments(target.path),
                target.query.map(str::to_string),
            )
        } else {
            (
                remove_dot_segments(&self.merge(target.path)),
                target.query.map(str::to_string),
            )
        };
        Ok(Self {
            scheme: self.scheme.clone(),
            userinfo: self.userinfo.clone(),
            host: self.host.clone(),
            port: self.port,
            path,
            query,
            fragment: target.fragment.map(str::to_string),
        })
    }

    /// The scheme, lowercase, such as `"https"`.
    ///
    /// # Returns
    ///
    /// The scheme without the colon.
    #[must_use]
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// The user information before the `@`, such as `"user:password"`.
    ///
    /// # Returns
    ///
    /// The raw user information, or `None` when the URL has none.
    #[must_use]
    pub fn userinfo(&self) -> Option<&str> {
        self.userinfo.as_deref()
    }

    /// The host, lowercase, with the brackets of an IPv6 literal (`"[::1]"`).
    ///
    /// # Returns
    ///
    /// The host, `Some("")` for an empty authority (`file:///etc`), or `None` when the URL has no
    /// authority at all (`mailto:a@b.c`).
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    /// The port written in the URL.
    ///
    /// # Returns
    ///
    /// The port, or `None` when the URL does not name one.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// The port to connect to: the one in the URL, or the default of a well-known scheme.
    ///
    /// # Returns
    ///
    /// The explicit port, else `80` for `http` and `ws`, `443` for `https` and `wss`, `21` for
    /// `ftp`, and `None` for any other scheme.
    #[must_use]
    pub fn port_or_default(&self) -> Option<u16> {
        self.port.or(match self.scheme.as_str() {
            "http" | "ws" => Some(80),
            "https" | "wss" => Some(443),
            "ftp" => Some(21),
            _ => None,
        })
    }

    /// The path, as written; possibly empty.
    ///
    /// # Returns
    ///
    /// The path, starting with `/` when the URL has an authority and a path.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The query, without the `?`.
    ///
    /// # Returns
    ///
    /// The raw query (see [`parse_query`](super::parse_query) to decode it), or `None`.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// The fragment, without the `#`.
    ///
    /// # Returns
    ///
    /// The raw fragment, or `None`.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }

    fn from_pieces(scheme: String, pieces: Pieces<'_>) -> Result<Self, UrlError> {
        let (userinfo, host, port) = match pieces.authority {
            Some(authority) => {
                let (userinfo, host, port) = split_authority(authority)?;
                (userinfo, Some(host), port)
            }
            None => (None, None, None),
        };
        Ok(Self {
            scheme,
            userinfo,
            host,
            port,
            path: pieces.path.to_string(),
            query: pieces.query.map(str::to_string),
            fragment: pieces.fragment.map(str::to_string),
        })
    }

    /// RFC 3986 section 5.2.3: the path of `relative` taken relative to this URL's path.
    fn merge(&self, relative: &str) -> String {
        if self.host.is_some() && self.path.is_empty() {
            return format!("/{relative}");
        }
        match self.path.rfind('/') {
            Some(slash) => format!("{}{relative}", &self.path[..=slash]),
            None => relative.to_string(),
        }
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("{}:", self.scheme);
        if let Some(host) = &self.host {
            out.push_str("//");
            if let Some(userinfo) = &self.userinfo {
                out.push_str(userinfo);
                out.push('@');
            }
            out.push_str(host);
            if let Some(port) = self.port {
                out.push(':');
                out.push_str(&port.to_string());
            }
        }
        out.push_str(&self.path);
        if let Some(query) = &self.query {
            out.push('?');
            out.push_str(query);
        }
        if let Some(fragment) = &self.fragment {
            out.push('#');
            out.push_str(fragment);
        }
        f.write_str(&out)
    }
}

impl FromStr for Url {
    type Err = UrlError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

/// Rejects the characters a URL cannot contain unencoded: space, controls and DEL.
fn check_characters(input: &str) -> Result<(), UrlError> {
    match input.bytes().position(|b| b <= 0x20 || b == 0x7f) {
        Some(index) => Err(UrlError::InvalidCharacter { index }),
        None => Ok(()),
    }
}

/// Whether the reference starts with `scheme:` (a `:` before any `/`, `?` or `#`).
fn has_scheme(reference: &str) -> bool {
    reference
        .find([':', '/', '?', '#'])
        .is_some_and(|i| reference.as_bytes()[i] == b':')
}

/// Splits `scheme:rest`, lowercasing and validating the scheme.
fn split_scheme(input: &str) -> Result<(String, &str), UrlError> {
    if !has_scheme(input) {
        return Err(UrlError::MissingScheme);
    }
    let (scheme, rest) = input.split_once(':').unwrap_or((input, ""));
    let bad = scheme.bytes().enumerate().position(|(i, b)| {
        if i == 0 {
            !b.is_ascii_alphabetic()
        } else {
            !(b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.'))
        }
    });
    match bad {
        Some(index) => Err(UrlError::InvalidScheme { index }),
        None if scheme.is_empty() => Err(UrlError::InvalidScheme { index: 0 }),
        None => Ok((scheme.to_ascii_lowercase(), rest)),
    }
}

/// Cuts everything after the scheme into authority, path, query and fragment.
fn split_pieces(rest: &str) -> Pieces<'_> {
    let (rest, fragment) = match rest.split_once('#') {
        Some((before, fragment)) => (before, Some(fragment)),
        None => (rest, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((before, query)) => (before, Some(query)),
        None => (rest, None),
    };
    let (authority, path) = match rest.strip_prefix("//") {
        Some(after) => match after.find('/') {
            Some(slash) => (Some(&after[..slash]), &after[slash..]),
            None => (Some(after), ""),
        },
        None => (None, rest),
    };
    Pieces {
        authority,
        path,
        query,
        fragment,
    }
}

/// Splits `userinfo@host:port`, validating the port and lowercasing the host.
fn split_authority(authority: &str) -> Result<(Option<String>, String, Option<u16>), UrlError> {
    let (userinfo, host_port) = match authority.rsplit_once('@') {
        Some((userinfo, host_port)) => (Some(userinfo.to_string()), host_port),
        None => (None, authority),
    };
    let (host, port) = if host_port.starts_with('[') {
        let close = host_port.find(']').ok_or(UrlError::InvalidHost)?;
        let (host, after) = host_port.split_at(close + 1);
        match after {
            "" => (host, ""),
            _ => (host, after.strip_prefix(':').ok_or(UrlError::InvalidHost)?),
        }
    } else {
        host_port.split_once(':').unwrap_or((host_port, ""))
    };
    let port = if port.is_empty() {
        None
    } else {
        Some(port.parse::<u16>().map_err(|_| UrlError::InvalidPort)?)
    };
    Ok((userinfo, host.to_ascii_lowercase(), port))
}

/// RFC 3986 section 5.2.4: resolves `.` and `..` segments of a path.
fn remove_dot_segments(path: &str) -> String {
    let absolute = path.starts_with('/');
    let segments: Vec<&str> = path.split('/').collect();
    let last = segments.len() - 1;
    let mut out: Vec<&str> = Vec::new();
    for (i, segment) in segments.iter().enumerate() {
        match *segment {
            "." => {
                if i == last {
                    out.push("");
                }
            }
            ".." => {
                if out.len() > usize::from(absolute) {
                    out.pop();
                }
                if i == last {
                    out.push("");
                }
            }
            other => out.push(other),
        }
    }
    out.join("/")
}

#[cfg(test)]
#[path = "parsed_url.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parsed_url.spec.rs"]
mod spec;
