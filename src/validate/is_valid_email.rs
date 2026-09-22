// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Checks a pragmatic subset of RFC 5322 that catches real typos, not a full grammar.
///
/// Requires exactly one `@`, a non-empty local part of ASCII letters, digits and `. _ % + -`
/// with no leading, trailing or doubled dot, and a domain of at least two dot-separated labels
/// (ASCII letters, digits and hyphens, none starting or ending with a hyphen) whose last label
/// is letters only. Quoted local parts, comments and internationalized domain names are not
/// supported: this is meant to reject obvious mistakes, not to be the final word on deliverability.
///
/// # Arguments
///
/// - `s` - The address to check.
///
/// # Returns
///
/// `true` when `s` passes the checks above.
///
/// # Examples
///
/// ```
/// use helpers4::validate::is_valid_email;
///
/// assert!(is_valid_email("jane.doe+list@example.co.uk"));
/// assert!(!is_valid_email("no-at-sign"));
/// assert!(!is_valid_email("@example.com"));
/// assert!(!is_valid_email("jane@localhost"));
/// ```
#[must_use]
pub fn is_valid_email(s: &str) -> bool {
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !domain.contains('@') && is_valid_local(local) && is_valid_domain(domain)
}

fn is_valid_local(local: &str) -> bool {
    !local.is_empty()
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && local
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b".+-_%".contains(&b))
}

fn is_valid_domain(domain: &str) -> bool {
    let labels: Vec<&str> = domain.split('.').collect();
    // `str::split` always yields at least one item, so a single label means there was no dot
    // at all (`"example"`) rather than an empty domain.
    if labels.len() < 2 {
        return false;
    }
    let (rest, tld) = labels.split_at(labels.len() - 1);
    let tld = tld[0];
    tld.len() >= 2
        && tld.bytes().all(|b| b.is_ascii_alphabetic())
        && rest.iter().all(|label| is_valid_label(label))
}

fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

#[cfg(test)]
#[path = "is_valid_email.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_valid_email.spec.rs"]
mod spec;
