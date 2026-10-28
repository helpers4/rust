// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn new_builds_a_request_of_normal_width() {
    let request = FontRequest::new(600.0, FontStyle::Italic);
    assert_eq!(
        (request.weight, request.style, request.stretch),
        (600.0, FontStyle::Italic, 100.0)
    );
}

#[test]
fn the_default_is_the_css_initial_value() {
    assert_eq!(
        FontRequest::default(),
        FontRequest::new(400.0, FontStyle::Normal)
    );
    assert_eq!(FontStyle::default(), FontStyle::Normal);
}
