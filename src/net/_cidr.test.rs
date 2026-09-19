// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn v4_membership_at_the_edges_of_a_block() {
    let inside = |ip: [u8; 4]| v4_in(Ipv4Addr::from(ip), [10, 0, 0, 0], 8);
    assert!(inside([10, 0, 0, 0]));
    assert!(inside([10, 255, 255, 255]));
    assert!(!inside([9, 255, 255, 255]));
    assert!(!inside([11, 0, 0, 0]));
}

#[test]
fn v4_prefix_zero_matches_everything_and_32_only_the_host() {
    assert!(v4_in(Ipv4Addr::new(8, 8, 8, 8), [0, 0, 0, 0], 0));
    assert!(v4_in(Ipv4Addr::new(1, 2, 3, 4), [1, 2, 3, 4], 32));
    assert!(!v4_in(Ipv4Addr::new(1, 2, 3, 5), [1, 2, 3, 4], 32));
}

#[test]
fn v6_membership_at_the_edges_of_a_block() {
    let base = 0x2001_0db8u128 << 96;
    assert!(v6_in("2001:db8::".parse().unwrap(), base, 32));
    assert!(v6_in(
        "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".parse().unwrap(),
        base,
        32
    ));
    assert!(!v6_in("2001:db9::".parse().unwrap(), base, 32));
    assert!(!v6_in(
        "2001:db7:ffff:ffff:ffff:ffff:ffff:ffff".parse().unwrap(),
        base,
        32
    ));
}

#[test]
fn v6_prefix_zero_matches_everything_and_128_only_the_host() {
    assert!(v6_in("::1".parse().unwrap(), 0, 0));
    assert!(v6_in("::1".parse().unwrap(), 1, 128));
    assert!(!v6_in("::2".parse().unwrap(), 1, 128));
}
