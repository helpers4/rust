// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::borrow::Cow;

/// Escapes the HTML special characters `&`, `<`, `>`, `"` and `'`.
///
/// Returns the input borrowed, without allocating, when there is nothing to escape. Use it to
/// embed untrusted text in HTML text nodes or quoted attribute values.
///
/// # Arguments
///
/// - `s` - The text to escape.
///
/// # Examples
///
/// ```
/// use helpers4::string::escape_html;
///
/// assert_eq!(
///     escape_html("<script>alert(\"xss\")</script>"),
///     "&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;"
/// );
/// assert_eq!(escape_html("It's a <test> & more"), "It&#39;s a &lt;test&gt; &amp; more");
/// assert_eq!(escape_html("plain"), "plain");
/// ```
#[must_use]
pub fn escape_html(s: &str) -> Cow<'_, str> {
    if !s.contains(['&', '<', '>', '"', '\'']) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    Cow::Owned(out)
}

#[cfg(test)]
#[path = "escape_html.test.rs"]
mod tests;

#[cfg(test)]
#[path = "escape_html.spec.rs"]
mod spec;
