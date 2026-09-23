// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn reads_every_conclusion_name() {
    for (name, status) in [
        ("success", Status::Success),
        ("passed", Status::Success),
        ("pass", Status::Success),
        ("failure", Status::Failure),
        ("failed", Status::Failure),
        ("error", Status::Failure),
        ("timed_out", Status::Failure),
        ("cancelled", Status::Cancelled),
        ("canceled", Status::Cancelled),
        ("skipped", Status::Skipped),
        ("pending", Status::Pending),
        ("queued", Status::Pending),
        ("in_progress", Status::Pending),
        ("running", Status::Pending),
    ] {
        assert_eq!(Status::from_conclusion(name), Some(status), "{name}");
        assert_eq!(Status::from_conclusion(&name.to_uppercase()), Some(status));
    }
}

#[test]
fn an_unknown_name_is_none() {
    assert_eq!(Status::from_conclusion("maybe"), None);
    assert_eq!(Status::from_conclusion(""), None);
    assert_eq!("maybe".parse::<Status>(), Err(()));
    assert_eq!("failure".parse::<Status>(), Ok(Status::Failure));
}

#[test]
fn every_status_has_an_icon_and_a_label() {
    let all = [
        Status::Success,
        Status::Failure,
        Status::Cancelled,
        Status::Skipped,
        Status::Pending,
    ];
    let icons: Vec<&str> = all.iter().map(|s| s.icon()).collect();
    assert_eq!(
        icons,
        [
            "\u{2705}",
            "\u{274c}",
            "\u{1f6ab}",
            "\u{23ed}\u{fe0f}",
            "\u{23f3}"
        ]
    );
    let labels: Vec<&str> = all.iter().map(|s| s.label()).collect();
    assert_eq!(
        labels,
        ["success", "failure", "cancelled", "skipped", "pending"]
    );
}

#[test]
fn a_label_reads_back_as_the_same_status() {
    for status in [
        Status::Success,
        Status::Failure,
        Status::Cancelled,
        Status::Skipped,
        Status::Pending,
    ] {
        assert_eq!(Status::from_conclusion(status.label()), Some(status));
    }
}
