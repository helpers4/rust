// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_one_row_per_job_plus_the_heading_and_the_table_header(names in prop::collection::vec("[a-z |]{0,6}", 0..6)) {
        let jobs: Vec<(&str, Status)> = names.iter().map(|n| (n.as_str(), Status::Success)).collect();
        let report = render_report("title", &jobs);
        prop_assert_eq!(report.lines().count(), jobs.len() + 4);
    }
}
