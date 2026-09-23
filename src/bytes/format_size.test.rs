// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn small_sizes_are_plain_bytes() {
    assert_eq!(format_size(0), "0 B");
    assert_eq!(format_size(1023), "1023 B");
}

#[test]
fn switches_to_kibibytes_at_1024() {
    assert_eq!(format_size(1024), "1 KiB");
    assert_eq!(format_size(1536), "1.5 KiB");
}

#[test]
fn rounds_to_the_nearest_tenth() {
    assert_eq!(format_size(1024 + 51), "1 KiB"); // 1.0498 -> 1.0
    assert_eq!(format_size(1024 + 52), "1.1 KiB"); // 1.0508 -> 1.1
}

#[test]
fn rounding_up_to_the_next_unit_reads_as_that_unit() {
    assert_eq!(format_size(1_048_575), "1 MiB");
    assert_eq!(format_size(1_048_576), "1 MiB");
}

#[test]
fn covers_every_unit() {
    assert_eq!(format_size(3 * 1024 * 1024), "3 MiB");
    assert_eq!(format_size(2 * 1024u64.pow(3)), "2 GiB");
    assert_eq!(format_size(1024u64.pow(4)), "1 TiB");
    assert_eq!(format_size(1024u64.pow(5)), "1 PiB");
    assert_eq!(format_size(1024u64.pow(6)), "1 EiB");
}

#[test]
fn the_largest_value_is_in_exbibytes() {
    assert_eq!(format_size(u64::MAX), "16 EiB");
}
