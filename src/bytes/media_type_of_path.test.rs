// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn finds_the_type_from_the_extension() {
    assert_eq!(media_type_of_path("layers/a@2x.webp"), Some("image/webp"));
    assert_eq!(media_type_of_path("a.svg"), Some("image/svg+xml"));
    assert_eq!(media_type_of_path("dir.v2/app.js"), Some("text/javascript"));
}

#[test]
fn ignores_case_the_query_string_and_the_fragment() {
    assert_eq!(
        media_type_of_path("/fonts/Inter.WOFF2?v=3"),
        Some("font/woff2")
    );
    assert_eq!(
        media_type_of_path("https://x.dev/a.png#top"),
        Some("image/png")
    );
}

#[test]
fn reads_windows_separators() {
    assert_eq!(media_type_of_path("C:\\art\\a.jpeg"), Some("image/jpeg"));
}

#[test]
fn gives_none_without_a_known_extension() {
    assert_eq!(media_type_of_path("LICENSE"), None);
    assert_eq!(media_type_of_path("a.unknown"), None);
    assert_eq!(media_type_of_path("dir.png/file"), None);
    assert_eq!(media_type_of_path(""), None);
}
