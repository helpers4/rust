// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

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
