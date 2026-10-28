// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::geometry::format_path;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(d in "\\PC{0,40}") {
        let _ = parse_path(&d);
    }

    #[test]
    fn reads_back_what_format_path_wrote(
        points in prop::collection::vec((-1000i32..1000, -1000i32..1000), 1..8)
    ) {
        let mut commands = vec![PathCommand::new('M', vec![f64::from(points[0].0), f64::from(points[0].1)])];
        for (x, y) in &points[1..] {
            commands.push(PathCommand::new('L', vec![f64::from(*x), f64::from(*y)]));
        }
        commands.push(PathCommand::new('Z', vec![]));
        prop_assert_eq!(parse_path(&format_path(&commands, 6)), Ok(commands));
    }
}
