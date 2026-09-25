// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

// RFC 4648 section 10 test vectors, exercising every one of the five padding lengths (0, 1, 3,
// 4 and 6 '=' characters).
#[test]
fn rfc4648_test_vectors() {
    assert_eq!(base32_encode(b""), "");
    assert_eq!(base32_encode(b"f"), "MY======");
    assert_eq!(base32_encode(b"fo"), "MZXQ====");
    assert_eq!(base32_encode(b"foo"), "MZXW6===");
    assert_eq!(base32_encode(b"foob"), "MZXW6YQ=");
    assert_eq!(base32_encode(b"fooba"), "MZXW6YTB");
    assert_eq!(base32_encode(b"foobar"), "MZXW6YTBOI======");
}
