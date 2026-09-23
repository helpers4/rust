// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn exposes_its_three_parts() {
    let info = LicenseInfo::new("MIT", "MIT License", Category::Permissive);
    assert_eq!(info.id(), "MIT");
    assert_eq!(info.name(), "MIT License");
    assert_eq!(info.category(), Category::Permissive);
}
