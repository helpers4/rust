// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::borrow::Cow;

/// Escapes the HTML special characters `&`, `<`, `>`, `"` and `'`.
///
/// Returns the input borrowed, without allocating, when there is nothing to escape. Use it to
/// embed untrusted text in HTML text nodes or quoted attribute values.
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
///
/// # Since
///
/// next
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
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn is_borrowed(c: Cow<'_, str>) -> bool {
        matches!(c, Cow::Borrowed(_))
    }

    #[test]
    fn escapes_all_five_characters() {
        assert_eq!(escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
    }

    #[test]
    fn keeps_other_characters() {
        assert_eq!(escape_html("a <b> é"), "a &lt;b&gt; é");
    }

    #[test]
    fn borrows_when_nothing_to_escape() {
        assert!(is_borrowed(escape_html("plain é")));
        assert!(!is_borrowed(escape_html("a&b")));
    }

    proptest! {
        #[test]
        fn output_has_no_raw_specials(s in ".*") {
            let out = escape_html(&s);
            prop_assert!(!out.contains(['<', '>', '"', '\'']));
        }

        #[test]
        fn borrowed_iff_input_has_no_specials(s in ".*") {
            let has_special = s.contains(['&', '<', '>', '"', '\'']);
            prop_assert_eq!(!is_borrowed(escape_html(&s)), has_special);
        }
    }
}
