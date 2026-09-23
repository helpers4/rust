// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_colors_and_styles() {
    assert_eq!(strip("\u{1b}[31mred\u{1b}[0m"), "red");
    assert_eq!(strip("\u{1b}[1;4;38;5;208mbold\u{1b}[m"), "bold");
    assert_eq!(strip("\u{1b}[38;2;1;2;3mrgb\u{1b}[0m"), "rgb");
}

#[test]
fn removes_cursor_and_erase_sequences() {
    assert_eq!(strip("a\u{1b}[2Kb\u{1b}[1;1Hc\u{1b}[?25lz"), "abcz");
}

#[test]
fn removes_operating_system_commands_ended_by_bel_or_st() {
    assert_eq!(strip("\u{1b}]0;window title\u{7}text"), "text");
    assert_eq!(strip("\u{1b}]0;window title\u{1b}\\text"), "text");
    assert_eq!(
        strip("\u{1b}]8;;https://x.org\u{7}link\u{1b}]8;;\u{7}"),
        "link"
    );
}

#[test]
fn an_osc_ended_by_an_escape_that_is_not_st_keeps_what_follows() {
    assert_eq!(strip("\u{1b}]0;t\u{1b}[31mred"), "[31mred");
}

#[test]
fn removes_two_character_escapes() {
    assert_eq!(strip("a\u{1b}cb\u{1b}7c\u{1b}8d"), "abcd");
    assert_eq!(strip("\u{1b}(Bx"), "x");
    assert_eq!(strip("\u{1b} Fx"), "x");
}

#[test]
fn a_lone_escape_is_dropped() {
    assert_eq!(strip("a\u{1b}"), "a");
    assert_eq!(strip("a\u{1b}é"), "aé");
    assert_eq!(strip("\u{1b}\u{1b}é"), "é");
}

#[test]
fn a_sequence_cut_short_disappears_with_the_rest() {
    assert_eq!(strip("ok\u{1b}[31"), "ok");
    assert_eq!(strip("ok\u{1b}]0;title"), "ok");
    assert_eq!(strip("ok\u{1b}("), "ok");
}

#[test]
fn keeps_everything_else_including_multibyte_text() {
    assert_eq!(strip("é\u{1b}[1m日本\u{1b}[0m✓"), "é日本✓");
    assert_eq!(strip("a\nb\tc"), "a\nb\tc");
}

#[test]
fn borrows_when_there_is_no_escape() {
    assert!(matches!(strip("plain"), Cow::Borrowed("plain")));
    assert!(matches!(strip(""), Cow::Borrowed("")));
}
