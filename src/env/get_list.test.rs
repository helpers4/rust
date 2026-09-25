// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn splits_and_trims_each_item() {
    assert_eq!(
        get_list("HOSTS=a, b ,c", "HOSTS", ',', &[]),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn a_single_item_is_a_one_element_list() {
    assert_eq!(
        get_list("HOSTS=only", "HOSTS", ',', &[]),
        vec!["only".to_string()]
    );
}

#[test]
fn empty_items_are_kept() {
    assert_eq!(
        get_list("HOSTS=a,,b", "HOSTS", ',', &[]),
        vec!["a".to_string(), String::new(), "b".to_string()]
    );
}

#[test]
fn missing_key_returns_the_default() {
    assert_eq!(
        get_list("", "HOSTS", ',', &["localhost"]),
        vec!["localhost".to_string()]
    );
}

#[test]
fn a_different_separator() {
    assert_eq!(
        get_list("PATH=/bin:/usr/bin", "PATH", ':', &[]),
        vec!["/bin".to_string(), "/usr/bin".to_string()]
    );
}
