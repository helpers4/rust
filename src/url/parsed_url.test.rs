// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn url(input: &str) -> Url {
    Url::parse(input).unwrap()
}

#[test]
fn parses_every_part() {
    let u = url("https://user:pw@Example.COM:8443/a/b?x=1&y=2#top");
    assert_eq!(u.scheme(), "https");
    assert_eq!(u.userinfo(), Some("user:pw"));
    assert_eq!(u.host(), Some("example.com"));
    assert_eq!(u.port(), Some(8443));
    assert_eq!(u.path(), "/a/b");
    assert_eq!(u.query(), Some("x=1&y=2"));
    assert_eq!(u.fragment(), Some("top"));
}

#[test]
fn every_part_but_the_scheme_and_path_is_optional() {
    let u = url("http://example.com");
    assert_eq!(u.userinfo(), None);
    assert_eq!(u.port(), None);
    assert_eq!(u.path(), "");
    assert_eq!(u.query(), None);
    assert_eq!(u.fragment(), None);
}

#[test]
fn lowercases_the_scheme() {
    assert_eq!(url("HTTP://x").scheme(), "http");
}

#[test]
fn accepts_a_url_without_an_authority() {
    let u = url("mailto:someone@example.com");
    assert_eq!(u.scheme(), "mailto");
    assert_eq!(u.host(), None);
    assert_eq!(u.path(), "someone@example.com");
    let u = url("urn:isbn:0451450523");
    assert_eq!(u.path(), "isbn:0451450523");
}

#[test]
fn an_empty_authority_is_an_empty_host() {
    let u = url("file:///etc/hosts");
    assert_eq!(u.host(), Some(""));
    assert_eq!(u.path(), "/etc/hosts");
}

#[test]
fn parses_ipv6_hosts() {
    let u = url("http://[::1]:8080/x");
    assert_eq!(u.host(), Some("[::1]"));
    assert_eq!(u.port(), Some(8080));
    assert_eq!(url("http://[::1]/x").port(), None);
    assert_eq!(url("http://[FE80::1]/").host(), Some("[fe80::1]"));
}

#[test]
fn an_empty_port_means_no_port() {
    assert_eq!(url("http://example.com:/x").port(), None);
}

#[test]
fn splits_the_userinfo_at_the_last_at_sign() {
    let u = url("http://a@b@example.com/");
    assert_eq!(u.userinfo(), Some("a@b"));
    assert_eq!(u.host(), Some("example.com"));
}

#[test]
fn a_question_mark_or_hash_ends_the_authority_and_path() {
    let u = url("http://example.com?q=1");
    assert_eq!(
        (u.host(), u.path(), u.query()),
        (Some("example.com"), "", Some("q=1"))
    );
    let u = url("http://example.com#f");
    assert_eq!((u.path(), u.fragment()), ("", Some("f")));
    let u = url("http://example.com/p#f?notquery");
    assert_eq!((u.query(), u.fragment()), (None, Some("f?notquery")));
}

#[test]
fn rejects_spaces_and_control_characters() {
    assert_eq!(
        Url::parse("http://exa mple.com"),
        Err(UrlError::InvalidCharacter { index: 10 })
    );
    assert_eq!(
        Url::parse("http://x/\n"),
        Err(UrlError::InvalidCharacter { index: 9 })
    );
    assert_eq!(
        Url::parse("http://x/\u{7f}"),
        Err(UrlError::InvalidCharacter { index: 9 })
    );
}

#[test]
fn rejects_a_missing_scheme() {
    assert_eq!(Url::parse("example.com"), Err(UrlError::MissingScheme));
    assert_eq!(Url::parse("/path:with-colon"), Err(UrlError::MissingScheme));
    assert_eq!(Url::parse("?q=a:b"), Err(UrlError::MissingScheme));
    assert_eq!(Url::parse(""), Err(UrlError::MissingScheme));
}

#[test]
fn rejects_an_invalid_scheme() {
    assert_eq!(
        Url::parse("1http://x"),
        Err(UrlError::InvalidScheme { index: 0 })
    );
    assert_eq!(
        Url::parse("://x"),
        Err(UrlError::InvalidScheme { index: 0 })
    );
    assert_eq!(
        Url::parse("ht_tp://x"),
        Err(UrlError::InvalidScheme { index: 2 })
    );
    assert_eq!(
        Url::parse("a+b-c.d://x").map(|u| u.scheme().to_string()),
        Ok("a+b-c.d".to_string())
    );
}

#[test]
fn rejects_a_malformed_host_or_port() {
    assert_eq!(Url::parse("http://[::1/x"), Err(UrlError::InvalidHost));
    assert_eq!(Url::parse("http://[::1]x/"), Err(UrlError::InvalidHost));
    assert_eq!(Url::parse("http://x:99999/"), Err(UrlError::InvalidPort));
    assert_eq!(Url::parse("http://x:abc/"), Err(UrlError::InvalidPort));
    assert_eq!(Url::parse("http://x:-1/"), Err(UrlError::InvalidPort));
    assert_eq!(Url::parse("http://[::1]:x/"), Err(UrlError::InvalidPort));
}

#[test]
fn knows_the_default_ports() {
    assert_eq!(url("http://x").port_or_default(), Some(80));
    assert_eq!(url("ws://x").port_or_default(), Some(80));
    assert_eq!(url("https://x").port_or_default(), Some(443));
    assert_eq!(url("wss://x").port_or_default(), Some(443));
    assert_eq!(url("ftp://x").port_or_default(), Some(21));
    assert_eq!(url("gopher://x").port_or_default(), None);
    assert_eq!(url("http://x:81").port_or_default(), Some(81));
}

#[test]
fn displays_back_the_normalized_url() {
    for text in [
        "https://user:pw@example.com:8443/a/b?x=1#top",
        "http://example.com",
        "mailto:someone@example.com",
        "file:///etc/hosts",
        "http://[::1]:8080/x",
        "http://example.com/?",
    ] {
        assert_eq!(url(text).to_string(), text);
    }
    assert_eq!(
        url("HTTP://EXAMPLE.com:80/A").to_string(),
        "http://example.com:80/A"
    );
}

#[test]
fn parses_through_from_str() {
    assert_eq!("http://x/".parse::<Url>(), Url::parse("http://x/"));
}

// RFC 3986, section 5.4: reference resolution examples against one base.
const BASE: &str = "http://a/b/c/d;p?q";

fn resolved(reference: &str) -> String {
    url(BASE).join(reference).unwrap().to_string()
}

#[test]
fn resolves_the_normal_examples_of_the_rfc() {
    for (reference, expected) in [
        ("g:h", "g:h"),
        ("g", "http://a/b/c/g"),
        ("./g", "http://a/b/c/g"),
        ("g/", "http://a/b/c/g/"),
        ("/g", "http://a/g"),
        ("//g", "http://g"),
        ("?y", "http://a/b/c/d;p?y"),
        ("g?y", "http://a/b/c/g?y"),
        ("#s", "http://a/b/c/d;p?q#s"),
        ("g#s", "http://a/b/c/g#s"),
        ("g?y#s", "http://a/b/c/g?y#s"),
        (";x", "http://a/b/c/;x"),
        ("g;x", "http://a/b/c/g;x"),
        ("g;x?y#s", "http://a/b/c/g;x?y#s"),
        ("", "http://a/b/c/d;p?q"),
        (".", "http://a/b/c/"),
        ("./", "http://a/b/c/"),
        ("..", "http://a/b/"),
        ("../", "http://a/b/"),
        ("../g", "http://a/b/g"),
        ("../..", "http://a/"),
        ("../../", "http://a/"),
        ("../../g", "http://a/g"),
    ] {
        assert_eq!(resolved(reference), expected, "reference {reference:?}");
    }
}

#[test]
fn resolves_the_abnormal_examples_of_the_rfc() {
    for (reference, expected) in [
        ("../../../g", "http://a/g"),
        ("../../../../g", "http://a/g"),
        ("/./g", "http://a/g"),
        ("/../g", "http://a/g"),
        ("g.", "http://a/b/c/g."),
        (".g", "http://a/b/c/.g"),
        ("g..", "http://a/b/c/g.."),
        ("..g", "http://a/b/c/..g"),
        ("./../g", "http://a/b/g"),
        ("./g/.", "http://a/b/c/g/"),
        ("g/./h", "http://a/b/c/g/h"),
        ("g/../h", "http://a/b/c/h"),
        ("g;x=1/./y", "http://a/b/c/g;x=1/y"),
        ("g;x=1/../y", "http://a/b/c/y"),
        ("g?y/./x", "http://a/b/c/g?y/./x"),
        ("g?y/../x", "http://a/b/c/g?y/../x"),
        ("g#s/./x", "http://a/b/c/g#s/./x"),
        ("g#s/../x", "http://a/b/c/g#s/../x"),
    ] {
        assert_eq!(resolved(reference), expected, "reference {reference:?}");
    }
}

#[test]
fn a_reference_with_an_authority_keeps_only_the_base_scheme() {
    assert_eq!(
        url("https://a/b").join("//c:81/d?e#f").unwrap().to_string(),
        "https://c:81/d?e#f"
    );
}

#[test]
fn joining_onto_a_url_with_an_empty_path_starts_at_the_root() {
    assert_eq!(url("http://a").join("b").unwrap().to_string(), "http://a/b");
}

#[test]
fn joining_onto_a_url_without_a_slash_in_its_path_replaces_the_path() {
    assert_eq!(
        url("mailto:a@b.c").join("d").unwrap().to_string(),
        "mailto:d"
    );
}

#[test]
fn keeps_the_userinfo_and_port_of_the_base() {
    assert_eq!(
        url("http://u:p@a:81/b/c").join("d").unwrap().to_string(),
        "http://u:p@a:81/b/d"
    );
}

#[test]
fn a_colon_after_a_slash_is_part_of_a_relative_path() {
    assert_eq!(resolved("g/h:i"), "http://a/b/c/g/h:i");
    assert_eq!(resolved("./g:h"), "http://a/b/c/g:h");
}

#[test]
fn joining_reports_the_errors_of_the_reference() {
    let base = url(BASE);
    assert_eq!(
        base.join("a b"),
        Err(UrlError::InvalidCharacter { index: 1 })
    );
    assert_eq!(base.join("1x:y"), Err(UrlError::InvalidScheme { index: 0 }));
    assert_eq!(base.join("//h:zz/"), Err(UrlError::InvalidPort));
}

#[test]
fn removes_dot_segments_of_relative_paths() {
    assert_eq!(remove_dot_segments("a/b/../c"), "a/c");
    assert_eq!(remove_dot_segments("a/../.."), "");
    assert_eq!(remove_dot_segments("a/b/."), "a/b/");
}
