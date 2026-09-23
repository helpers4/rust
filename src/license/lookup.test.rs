// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn finds_a_license_by_its_identifier() {
    let info = lookup("MIT").unwrap();
    assert_eq!(
        (info.id(), info.name(), info.category()),
        ("MIT", "MIT License", Category::Permissive)
    );
}

#[test]
fn ignores_case_and_answers_in_the_canonical_case() {
    assert_eq!(lookup("apache-2.0").map(|i| i.id()), Some("Apache-2.0"));
    assert_eq!(lookup("BSD-3-CLAUSE").map(|i| i.id()), Some("BSD-3-Clause"));
}

#[test]
fn accepts_deprecated_identifiers() {
    assert_eq!(lookup("GPL-3.0").map(|i| i.id()), Some("GPL-3.0-only"));
    assert_eq!(lookup("GPL-3.0+").map(|i| i.id()), Some("GPL-3.0-or-later"));
    assert_eq!(
        lookup("lgpl-2.1+").map(|i| i.id()),
        Some("LGPL-2.1-or-later")
    );
    assert_eq!(lookup("AGPL-3.0").map(|i| i.id()), Some("AGPL-3.0-only"));
}

#[test]
fn knows_the_families() {
    let category = |id| lookup(id).map(|i| i.category());
    assert_eq!(category("Apache-2.0"), Some(Category::Permissive));
    assert_eq!(category("Unlicense"), Some(Category::PublicDomain));
    assert_eq!(category("CC0-1.0"), Some(Category::PublicDomain));
    assert_eq!(category("MPL-2.0"), Some(Category::WeakCopyleft));
    assert_eq!(category("LGPL-3.0-or-later"), Some(Category::WeakCopyleft));
    assert_eq!(category("GPL-2.0-only"), Some(Category::StrongCopyleft));
    assert_eq!(category("AGPL-3.0-only"), Some(Category::NetworkCopyleft));
    assert_eq!(category("SSPL-1.0"), Some(Category::NetworkCopyleft));
}

#[test]
fn an_unknown_identifier_is_none() {
    assert_eq!(lookup("Not-A-License"), None);
    assert_eq!(lookup(""), None);
    assert_eq!(lookup("MIT OR Apache-2.0"), None);
}

#[test]
fn every_table_entry_can_be_found_by_its_own_identifier() {
    for (id, name, category) in crate::license::_table::TABLE {
        let info = lookup(id).unwrap();
        assert_eq!(
            (info.id(), info.name(), info.category()),
            (id, name, category)
        );
    }
}
