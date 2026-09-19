// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn public(s: &str) -> bool {
    is_public_ip(s.parse().unwrap())
}

#[test]
fn ipv4_non_public_blocks_are_refused_at_both_ends() {
    for s in [
        "0.0.0.0",
        "0.255.255.255",
        "10.0.0.0",
        "10.255.255.255",
        "100.64.0.0",
        "100.127.255.255",
        "127.0.0.1",
        "127.255.255.255",
        "169.254.0.0",
        "169.254.169.254",
        "172.16.0.0",
        "172.31.255.255",
        "192.0.0.0",
        "192.0.0.255",
        "192.0.2.1",
        "192.88.99.1",
        "192.168.0.0",
        "192.168.255.255",
        "198.18.0.0",
        "198.19.255.255",
        "198.51.100.5",
        "203.0.113.9",
        "224.0.0.1",
        "239.255.255.255",
        "240.0.0.1",
        "255.255.255.255",
    ] {
        assert!(!public(s), "{s} must not be public");
    }
}

#[test]
fn ipv4_addresses_just_outside_the_blocks_are_public() {
    for s in [
        "1.1.1.1",
        "8.8.8.8",
        "9.255.255.255",
        "11.0.0.0",
        "100.63.255.255",
        "100.128.0.0",
        "126.255.255.255",
        "128.0.0.0",
        "169.253.255.255",
        "169.255.0.0",
        "172.15.255.255",
        "172.32.0.0",
        "192.0.1.0",
        "192.167.255.255",
        "192.169.0.0",
        "198.17.255.255",
        "198.20.0.0",
        "223.255.255.255",
    ] {
        assert!(public(s), "{s} must be public");
    }
}

#[test]
fn ipv6_non_public_addresses_are_refused() {
    for s in [
        "::",
        "::1",
        "::8.8.8.8",
        "fc00::1",
        "fdff::1",
        "fe80::1",
        "fec0::1",
        "ff02::1",
        "2001::1",
        "2001:1ff:ffff:ffff:ffff:ffff:ffff:ffff",
        "2001:db8::1",
        "2002:a9fe:a9fe::1",
        "2002:808:808::1",
        "3fff::1",
        "3fff:fff:ffff::1",
        "4000::1",
        "100::1",
    ] {
        assert!(!public(s), "{s} must not be public");
    }
}

#[test]
fn ipv6_global_unicast_is_public() {
    for s in [
        "2000::1",
        "2001:200::1",
        "2400:cb00::1",
        "2606:4700:4700::1111",
        "2a00:1450:4001::1",
        "3ffe::1",
    ] {
        assert!(public(s), "{s} must be public");
    }
}

#[test]
fn ipv4_mapped_addresses_follow_the_embedded_ipv4() {
    assert!(public("::ffff:8.8.8.8"));
    for s in [
        "::ffff:10.0.0.1",
        "::ffff:127.0.0.1",
        "::ffff:169.254.169.254",
    ] {
        assert!(!public(s), "{s} must not be public");
    }
}

#[test]
fn nat64_addresses_follow_the_embedded_ipv4() {
    assert!(public("64:ff9b::808:808"));
    for s in ["64:ff9b::a9fe:a9fe", "64:ff9b::a00:1", "64:ff9b::7f00:1"] {
        assert!(!public(s), "{s} must not be public");
    }
}
