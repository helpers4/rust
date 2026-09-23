// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn writes_a_heading_and_one_row_per_job() {
    let report = render_report(
        "Validation",
        &[("build", Status::Success), ("tests", Status::Failure)],
    );
    assert_eq!(
        report,
        "### \u{274c} Validation\n\n| | Job | Status |\n|:-:|-----|:------:|\n| \u{2705} | build | `success` |\n| \u{274c} | tests | `failure` |"
    );
}

#[test]
fn the_heading_shows_the_overall_status() {
    assert!(render_report("t", &[("a", Status::Success)]).starts_with("### \u{2705} t"));
    assert!(render_report("t", &[("a", Status::Pending)]).starts_with("### \u{23f3} t"));
    assert!(render_report("t", &[]).starts_with("### \u{23ed}\u{fe0f} t"));
}

#[test]
fn no_jobs_gives_just_the_header_of_the_table() {
    assert_eq!(
        render_report("t", &[]),
        "### \u{23ed}\u{fe0f} t\n\n| | Job | Status |\n|:-:|-----|:------:|"
    );
}

#[test]
fn keeps_job_names_and_titles_from_breaking_the_table() {
    let report = render_report("a\nb", &[("x|y\nz", Status::Skipped)]);
    assert!(report.starts_with("### \u{23ed}\u{fe0f} a b\n"));
    assert!(report.ends_with("| \u{23ed}\u{fe0f} | x\\|y z | `skipped` |"));
}
