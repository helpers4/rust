// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::collections::HashSet;

#[test]
fn every_identifier_appears_once() {
    let ids: HashSet<&str> = TABLE.iter().map(|(id, _, _)| *id).collect();
    assert_eq!(ids.len(), TABLE.len());
}

#[test]
fn identifiers_only_use_the_spdx_characters() {
    for (id, _, _) in TABLE {
        assert!(
            id.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'.'),
            "{id}"
        );
    }
}

#[test]
fn every_deprecated_identifier_points_at_a_known_one() {
    for (old, new) in DEPRECATED {
        assert!(TABLE.iter().any(|(id, _, _)| *id == new), "{old} -> {new}");
        assert!(
            !TABLE.iter().any(|(id, _, _)| *id == old),
            "{old} is still in the table"
        );
    }
}

#[test]
fn every_license_has_a_name() {
    assert!(TABLE.iter().all(|(_, name, _)| !name.is_empty()));
}
