// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use super::_cidr::{v4_in, v6_in};

/// Returns `true` when `ip` is a globally reachable unicast address, `false` for everything
/// that is loopback, private, link-local, shared, documentation, reserved or otherwise not on the
/// public internet.
///
/// It is meant as the address check of an SSRF guard (a server that fetches user-supplied URLs).
/// The guarantee is one-sided: **no address that the IANA special-purpose registries mark as not
/// globally reachable is reported public**. Where a block is refused as a whole and the registry
/// carves a reachable piece out of it, the whole block is refused on purpose.
///
/// - **IPv4** is refused when it falls in any block of the IANA IPv4 special-purpose registry
///   that is not globally reachable: `0.0.0.0/8`, `10.0.0.0/8`, `100.64.0.0/10` (shared address
///   space, e.g. carrier-grade NAT and Tailscale), `127.0.0.0/8`, `169.254.0.0/16` (link-local,
///   including the cloud metadata address), `172.16.0.0/12`, `192.0.0.0/24` (including the two
///   anycast addresses `192.0.0.9` and `192.0.0.10` that the registry lists as reachable),
///   `192.0.2.0/24`, `192.88.99.0/24`, `192.168.0.0/16`, `198.18.0.0/15`, `198.51.100.0/24`,
///   `203.0.113.0/24`, and `224.0.0.0/3` (multicast, reserved and broadcast).
/// - **IPv6** is only accepted inside the global unicast space `2000::/3`, minus `2001::/23`
///   (IETF assignments, including Teredo), `2001:db8::/32` and `3fff::/20` (documentation, RFC
///   9637) and `2002::/16` (6to4). Loopback, unspecified, unique-local, link-local, multicast and
///   the deprecated IPv4-compatible `::a.b.c.d` form are all outside `2000::/3`. Everything else
///   inside `2000::/3` is accepted, **including blocks IANA holds in reserve** (`2d00::/8` to
///   `3e00::/8`, `3f00::/9` to `3ffe::/16`): nothing is routed there today, and IANA keeps
///   allocating out of this space, so a block delegated tomorrow must not start failing the guard.
/// - **Embedded IPv4** is judged by the IPv4 rules: an IPv4-mapped address (`::ffff:a.b.c.d`) and
///   an address under the NAT64 well-known prefix (`64:ff9b::/96`) are public exactly when the
///   IPv4 address inside is. 6to4 addresses are refused outright.
///
/// # What this check cannot know
///
/// This is only one half of an SSRF defence.
///
/// - **The address that is really used.** Resolve the name once, validate that address, and
///   connect to it (not to the name again, which invites DNS rebinding), and validate every
///   redirect target the same way.
/// - **What is not an IP literal to `std`.** Spellings such as `2130706433`, `0x7f.1` or
///   `127.1` do not parse as an [`IpAddr`], yet a URL parser reads them as `127.0.0.1`. Parse the
///   URL first and pass the address it produces here (see also
///   [`is_valid_hostname`](super::is_valid_hostname), which rejects them).
/// - **Provider-specific addresses inside a public range.** A registry cannot say that, for
///   example, Azure's wire server `168.63.129.16` is internal: it is reported public.
///
/// # Arguments
///
/// - `ip` - The address to check.
///
/// # Examples
///
/// ```
/// use helpers4::net::is_public_ip;
///
/// assert!(is_public_ip("8.8.8.8".parse().unwrap()));
/// assert!(is_public_ip("2606:4700:4700::1111".parse().unwrap()));
///
/// assert!(!is_public_ip("169.254.169.254".parse().unwrap())); // cloud metadata
/// assert!(!is_public_ip("::1".parse().unwrap()));
/// assert!(!is_public_ip("::ffff:10.0.0.1".parse().unwrap())); // private, in IPv6 clothes
/// ```
#[must_use]
pub fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_public_v4(v4),
        IpAddr::V6(v6) => is_public_v6(v6),
    }
}

/// IANA IPv4 special-purpose blocks that are not globally reachable.
const V4_NON_PUBLIC: &[([u8; 4], u32)] = &[
    ([0, 0, 0, 0], 8),
    ([10, 0, 0, 0], 8),
    ([100, 64, 0, 0], 10),
    ([127, 0, 0, 0], 8),
    ([169, 254, 0, 0], 16),
    ([172, 16, 0, 0], 12),
    ([192, 0, 0, 0], 24),
    ([192, 0, 2, 0], 24),
    ([192, 88, 99, 0], 24),
    ([192, 168, 0, 0], 16),
    ([198, 18, 0, 0], 15),
    ([198, 51, 100, 0], 24),
    ([203, 0, 113, 0], 24),
    ([224, 0, 0, 0], 3),
];

/// Blocks inside the global unicast space `2000::/3` that are not public.
const V6_NON_PUBLIC: &[(u128, u32)] = &[
    (0x2001_0000 << 96, 23),
    (0x2001_0db8 << 96, 32),
    (0x2002 << 112, 16),
    (0x3fff << 112, 20),
];

/// The NAT64 well-known prefix `64:ff9b::/96` (RFC 6052).
const NAT64: u128 = 0x0064_ff9b << 96;

fn is_public_v4(ip: Ipv4Addr) -> bool {
    !V4_NON_PUBLIC
        .iter()
        .any(|&(base, prefix)| v4_in(ip, base, prefix))
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return is_public_v4(mapped);
    }
    if v6_in(ip, NAT64, 96) {
        let octets = ip.octets();
        return is_public_v4(Ipv4Addr::new(
            octets[12], octets[13], octets[14], octets[15],
        ));
    }
    v6_in(ip, 0x2000 << 112, 3)
        && !V6_NON_PUBLIC
            .iter()
            .any(|&(base, prefix)| v6_in(ip, base, prefix))
}

#[cfg(test)]
#[path = "is_public_ip.test.rs"]
mod tests;

#[cfg(test)]
#[path = "is_public_ip.spec.rs"]
mod spec;
