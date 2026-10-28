// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn ignores_spaces_and_case() {
    assert!(same_family("SF Pro", "sfpro"));
    assert!(same_family("  Neutrif  Pro ", "NEUTRIFPRO"));
    assert!(same_family("", "  "));
}

#[test]
fn different_names_differ() {
    assert!(!same_family("SF Pro", "SF Mono"));
    assert!(!same_family("Inter", "Inter Tight"));
}
