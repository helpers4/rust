// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn maps_every_deprecated_identifier() {
    assert_eq!(normalize("GPL-2.0"), Some("GPL-2.0-only"));
    assert_eq!(normalize("GPL-2.0+"), Some("GPL-2.0-or-later"));
    assert_eq!(normalize("GPL-3.0"), Some("GPL-3.0-only"));
    assert_eq!(normalize("GPL-3.0+"), Some("GPL-3.0-or-later"));
    assert_eq!(normalize("LGPL-2.1"), Some("LGPL-2.1-only"));
    assert_eq!(normalize("LGPL-2.1+"), Some("LGPL-2.1-or-later"));
    assert_eq!(normalize("LGPL-3.0"), Some("LGPL-3.0-only"));
    assert_eq!(normalize("LGPL-3.0+"), Some("LGPL-3.0-or-later"));
    assert_eq!(normalize("AGPL-3.0"), Some("AGPL-3.0-only"));
    assert_eq!(normalize("AGPL-3.0+"), Some("AGPL-3.0-or-later"));
}

#[test]
fn ignores_case() {
    assert_eq!(normalize("gpl-3.0"), Some("GPL-3.0-only"));
}

#[test]
fn current_and_unknown_identifiers_give_none() {
    assert_eq!(normalize("GPL-3.0-only"), None);
    assert_eq!(normalize("MIT"), None);
    assert_eq!(normalize(""), None);
}
