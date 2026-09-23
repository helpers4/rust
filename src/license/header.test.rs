// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn writes_the_copyright_and_the_identifier() {
    assert_eq!(
        header("Jane Doe", "2025", "MIT"),
        "Copyright (C) 2025 Jane Doe\nSPDX-License-Identifier: MIT"
    );
}

#[test]
fn accepts_a_range_of_years_and_an_expression() {
    assert_eq!(
        header("ACME", "2020-2025", "MIT OR Apache-2.0"),
        "Copyright (C) 2020-2025 ACME\nSPDX-License-Identifier: MIT OR Apache-2.0"
    );
}

#[test]
fn has_no_trailing_newline() {
    assert!(!header("a", "1", "MIT").ends_with('\n'));
}
