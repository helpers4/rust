// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn commit(message: &str) -> Commit {
    Commit::parse(message).unwrap()
}

#[test]
fn parses_a_simple_header() {
    let c = commit("fix: correct the off-by-one");
    assert_eq!(c.kind(), "fix");
    assert_eq!(c.scope(), None);
    assert!(!c.is_breaking());
    assert_eq!(c.description(), "correct the off-by-one");
    assert_eq!((c.body(), c.footers().len()), (None, 0));
}

#[test]
fn parses_a_scope() {
    let c = commit("feat(parser): allow trailing commas");
    assert_eq!((c.kind(), c.scope()), ("feat", Some("parser")));
    assert_eq!(commit("chore(lib-rust): x").scope(), Some("lib-rust"));
    assert_eq!(commit("chore(a b): x").scope(), Some("a b"));
}

#[test]
fn a_bang_marks_a_breaking_change() {
    assert!(commit("feat!: drop support").is_breaking());
    let c = commit("feat(api)!: drop v1");
    assert!(c.is_breaking());
    assert_eq!(c.scope(), Some("api"));
}

#[test]
fn keeps_an_emoji_in_the_description() {
    assert_eq!(
        commit("feat(number): \u{2728} add the number module").description(),
        "\u{2728} add the number module"
    );
}

#[test]
fn parses_a_body() {
    let c = commit("fix: a\n\nfirst line\nsecond line\n\nsecond paragraph");
    assert_eq!(
        c.body(),
        Some("first line\nsecond line\n\nsecond paragraph")
    );
}

#[test]
fn parses_footers_with_either_separator() {
    let c = commit("fix: a\n\nReviewed-by: Z\nRefs #133\nCloses: #4");
    assert_eq!(
        c.footers(),
        [
            ("Reviewed-by".to_string(), "Z".to_string()),
            ("Refs".to_string(), "133".to_string()),
            ("Closes".to_string(), "#4".to_string()),
        ]
    );
    assert_eq!(c.body(), None);
}

#[test]
fn separates_the_body_from_the_footers() {
    let c = commit("fix: a\n\nThe body.\n\nRefs #1");
    assert_eq!(c.body(), Some("The body."));
    assert_eq!(c.footer("Refs"), Some("1"));
}

#[test]
fn a_footer_value_may_span_lines() {
    let c =
        commit("fix: a\n\nBREAKING CHANGE: first\nmore of it\n\nstill the same footer\nRefs #2");
    assert_eq!(
        c.footer("BREAKING CHANGE"),
        Some("first\nmore of it\n\nstill the same footer")
    );
    assert_eq!(c.footer("Refs"), Some("2"));
}

#[test]
fn a_breaking_change_footer_makes_the_commit_breaking() {
    let c = commit("feat: a\n\nBREAKING CHANGE: the API changed");
    assert!(c.is_breaking());
    assert_eq!(c.footer("BREAKING CHANGE"), Some("the API changed"));
    assert!(commit("feat: a\n\nBREAKING-CHANGE: x").is_breaking());
}

#[test]
fn footer_lookup_ignores_case_and_returns_the_first() {
    let c = commit("fix: a\n\nrefs: 1\nRefs: 2");
    assert_eq!(c.footer("REFS"), Some("1"));
    assert_eq!(c.footer("missing"), None);
}

#[test]
fn a_colon_line_in_the_middle_of_a_paragraph_is_not_a_footer() {
    let c = commit("fix: a\n\nfirst\nNote: still the body\nlast");
    assert_eq!(c.body(), Some("first\nNote: still the body\nlast"));
    assert!(c.footers().is_empty());
}

#[test]
fn tolerates_windows_line_endings_and_trailing_whitespace() {
    let c = commit("fix: a\r\n\r\nbody\r\n\r\nRefs #1\r\n\n\n");
    assert_eq!(
        (c.description(), c.body(), c.footer("Refs")),
        ("a", Some("body"), Some("1"))
    );
}

#[test]
fn several_blank_lines_before_the_body_are_fine() {
    assert_eq!(commit("fix: a\n\n\n\nbody").body(), Some("body"));
}

#[test]
fn rejects_an_empty_message() {
    assert_eq!(Commit::parse(""), Err(ParseCommitError::Empty));
    assert_eq!(Commit::parse("  \n\n"), Err(ParseCommitError::Empty));
}

#[test]
fn rejects_a_header_without_the_separator() {
    assert_eq!(
        Commit::parse("just a sentence"),
        Err(ParseCommitError::MissingSeparator)
    );
    assert_eq!(
        Commit::parse("feat:no space"),
        Err(ParseCommitError::MissingSeparator)
    );
}

#[test]
fn rejects_an_empty_description() {
    assert_eq!(
        Commit::parse("feat:"),
        Err(ParseCommitError::EmptyDescription)
    );
    assert_eq!(
        Commit::parse("feat:  "),
        Err(ParseCommitError::EmptyDescription)
    );
    assert_eq!(
        Commit::parse("feat: \n\nbody"),
        Err(ParseCommitError::EmptyDescription)
    );
}

#[test]
fn rejects_an_invalid_type() {
    assert_eq!(
        Commit::parse(": description"),
        Err(ParseCommitError::InvalidType)
    );
    assert_eq!(
        Commit::parse("fe at: d"),
        Err(ParseCommitError::InvalidType)
    );
    assert_eq!(
        Commit::parse("(scope): d"),
        Err(ParseCommitError::InvalidType)
    );
    assert_eq!(Commit::parse("!: d"), Err(ParseCommitError::InvalidType));
}

#[test]
fn rejects_an_invalid_scope() {
    assert_eq!(
        Commit::parse("feat(): d"),
        Err(ParseCommitError::InvalidScope)
    );
    assert_eq!(
        Commit::parse("feat(a: d"),
        Err(ParseCommitError::InvalidScope)
    );
    assert_eq!(
        Commit::parse("feat(a(b)): d"),
        Err(ParseCommitError::InvalidScope)
    );
    assert_eq!(
        Commit::parse("feat(a)b: d"),
        Err(ParseCommitError::InvalidScope)
    );
}

#[test]
fn rejects_a_body_without_the_blank_line() {
    assert_eq!(
        Commit::parse("fix: a\nbody right away"),
        Err(ParseCommitError::MissingBlankLine)
    );
}

#[test]
fn the_bump_follows_the_type_and_breaking_flag() {
    assert_eq!(commit("feat: a").bump(), Bump::Minor);
    assert_eq!(commit("FEAT: a").bump(), Bump::Minor);
    assert_eq!(commit("fix: a").bump(), Bump::Patch);
    assert_eq!(commit("Fix: a").bump(), Bump::Patch);
    assert_eq!(commit("docs: a").bump(), Bump::None);
    assert_eq!(commit("fix!: a").bump(), Bump::Major);
    assert_eq!(commit("docs: a\n\nBREAKING CHANGE: b").bump(), Bump::Major);
}

#[test]
fn displays_the_message_back() {
    assert_eq!(
        commit("feat(api)!: drop v1").to_string(),
        "feat(api)!: drop v1"
    );
    assert_eq!(commit("fix: a").to_string(), "fix: a");
    assert_eq!(
        commit("fix: a\n\nthe body\n\nRefs #1\nReviewed-by: Z").to_string(),
        "fix: a\n\nthe body\n\nRefs: 1\nReviewed-by: Z"
    );
    assert_eq!(
        commit("feat: a\n\nBREAKING CHANGE: b").to_string(),
        "feat: a\n\nBREAKING CHANGE: b"
    );
    assert_eq!("fix: a".parse::<Commit>(), Commit::parse("fix: a"));
}

#[test]
fn split_footer_rejects_lines_that_are_not_footers() {
    assert_eq!(split_footer("plain text"), None);
    assert_eq!(split_footer(": no token"), None);
    assert_eq!(split_footer("Token:no space"), None);
    assert_eq!(split_footer("no separator at all"), None);
    assert_eq!(split_footer("Token"), None);
    assert_eq!(split_footer("BREAKING CHANGE:"), None);
}
