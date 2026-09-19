// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Converts `s` into a lowercase, hyphen-separated slug safe for URLs.
///
/// Letters and digits (Unicode included) are kept and lowercased, apostrophes are dropped, and
/// every other run of characters becomes a single hyphen; leading and trailing hyphens are
/// never produced. Diacritics are **not** stripped: `"café"` stays `"café"`.
///
/// # Examples
///
/// ```
/// use helpers4::string::slugify;
///
/// assert_eq!(slugify("Hello World!"), "hello-world");
/// assert_eq!(slugify("  It's  a --- test "), "its-a-test");
/// assert_eq!(slugify("!!!"), "");
/// ```
///
/// # Since
///
/// next
pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_hyphen = false;
    for c in s.chars() {
        if c == '\'' || c == '\u{2019}' {
            continue;
        }
        if c.is_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.extend(c.to_lowercase());
        } else {
            pending_hyphen = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn slugifies_plain_text() {
        assert_eq!(slugify("Hello World!"), "hello-world");
    }

    #[test]
    fn collapses_runs_and_trims_edges() {
        assert_eq!(slugify("  a --- b  "), "a-b");
        assert_eq!(slugify("-a"), "a");
    }

    #[test]
    fn drops_ascii_and_typographic_apostrophes() {
        assert_eq!(slugify("It's"), "its");
        assert_eq!(slugify("It\u{2019}s"), "its");
    }

    #[test]
    fn keeps_unicode_letters() {
        assert_eq!(slugify("Café Été"), "café-été");
    }

    #[test]
    fn nothing_alphanumeric_gives_empty() {
        assert_eq!(slugify(""), "");
        assert_eq!(slugify("!!!"), "");
    }

    proptest! {
        #[test]
        fn is_idempotent_on_ascii(s in "[ -~]*") {
            let once = slugify(&s);
            prop_assert_eq!(slugify(&once), once);
        }

        #[test]
        fn ascii_output_is_url_safe(s in "[ -~]*") {
            let out = slugify(&s);
            prop_assert!(out.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
            prop_assert!(!out.starts_with('-') && !out.ends_with('-') && !out.contains("--"));
        }
    }
}
