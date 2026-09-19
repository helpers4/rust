// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn std_non_public_ipv4_predicates_imply_not_public(bits in any::<u32>()) {
        let ip = Ipv4Addr::from(bits);
        if ip.is_private() || ip.is_loopback() || ip.is_link_local() || ip.is_multicast()
            || ip.is_broadcast() || ip.is_documentation() || ip.is_unspecified()
        {
            prop_assert!(!is_public_ip(IpAddr::V4(ip)));
        }
    }

    #[test]
    fn ipv4_mapped_agrees_with_the_plain_ipv4(bits in any::<u32>()) {
        let ip = Ipv4Addr::from(bits);
        prop_assert_eq!(is_public_ip(IpAddr::V6(ip.to_ipv6_mapped())), is_public_ip(IpAddr::V4(ip)));
    }

    #[test]
    fn nat64_agrees_with_the_embedded_ipv4(bits in any::<u32>()) {
        let ip = Ipv4Addr::from(bits);
        let nat64 = Ipv6Addr::from(NAT64 | u128::from(bits));
        prop_assert_eq!(is_public_ip(IpAddr::V6(nat64)), is_public_ip(IpAddr::V4(ip)));
    }

    #[test]
    fn six_to_four_is_never_public(rest in any::<u128>()) {
        let ip = Ipv6Addr::from((0x2002 << 112) | (rest >> 16));
        prop_assert!(!is_public_ip(IpAddr::V6(ip)));
    }

    #[test]
    fn std_non_public_ipv6_predicates_imply_not_public(bits in any::<u128>()) {
        let ip = Ipv6Addr::from(bits);
        let first = ip.segments()[0];
        let unique_local = first & 0xfe00 == 0xfc00;
        let link_local = first & 0xffc0 == 0xfe80;
        if ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() || unique_local || link_local {
            prop_assert!(!is_public_ip(IpAddr::V6(ip)));
        }
    }

    #[test]
    fn only_global_unicast_can_be_public(bits in any::<u128>()) {
        let ip = Ipv6Addr::from(bits);
        let embedded = ip.to_ipv4_mapped().is_some() || v6_in(ip, NAT64, 96);
        if is_public_ip(IpAddr::V6(ip)) && !embedded {
            prop_assert_eq!(bits >> 125, 0b001);
        }
    }
}
