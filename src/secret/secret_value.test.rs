// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_and_debug_hide_the_value() {
    let secret = Secret::new("hunter2".to_string());
    assert_eq!(secret.to_string(), "[REDACTED]");
    assert_eq!(format!("{secret:?}"), "Secret([REDACTED])");
    assert!(!format!("{secret:#?}").contains("hunter2"));
}

#[test]
fn the_value_is_read_explicitly() {
    let secret = Secret::new(String::from("hunter2"));
    assert_eq!(secret.expose(), "hunter2");
    assert_eq!(secret.into_inner(), "hunter2");
}

#[test]
fn it_can_wrap_any_type_and_be_built_with_from() {
    let key: Secret<[u8; 2]> = [1, 2].into();
    assert_eq!(key.expose(), &[1, 2]);
    assert_eq!(Secret::from(7u32).into_inner(), 7);
}

#[test]
fn a_secret_inside_a_struct_stays_hidden_in_its_debug_output() {
    // The fields are only read through the derived `Debug`, which is the point of the test.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Config {
        user: String,
        password: Secret<String>,
    }
    let config = Config {
        user: "admin".to_string(),
        password: Secret::new("hunter2".to_string()),
    };
    let text = format!("{config:?}");
    assert!(text.contains("admin") && !text.contains("hunter2"));
}

#[test]
fn cloning_keeps_the_value() {
    let secret = Secret::new(String::from("x"));
    assert_eq!(secret.clone().expose(), "x");
}
