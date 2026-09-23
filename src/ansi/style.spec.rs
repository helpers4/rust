// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn any_color() -> impl Strategy<Value = Color> {
    prop_oneof![
        Just(Color::Red),
        Just(Color::BrightCyan),
        any::<u8>().prop_map(Color::Ansi256),
        (any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(r, g, b)| Color::Rgb(r, g, b)),
    ]
}

proptest! {
    #[test]
    fn stripping_gives_back_the_text(
        text in "[a-z é日\\n]{0,20}",
        fg in proptest::option::of(any_color()),
        bg in proptest::option::of(any_color()),
        bold in any::<bool>(),
    ) {
        let mut style = Style::new();
        if let Some(color) = fg { style = style.fg(color); }
        if let Some(color) = bg { style = style.bg(color); }
        if bold { style = style.bold(); }
        let painted = style.paint(&text);
        prop_assert_eq!(crate::ansi::strip(&painted), text.as_str());
    }
}
