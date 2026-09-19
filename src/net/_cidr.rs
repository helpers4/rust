// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! CIDR membership tests shared by the `net` helpers. Not re-exported.

use std::net::{Ipv4Addr, Ipv6Addr};

/// Whether `ip` is inside `base/prefix` (`prefix` in `0..=32`).
pub(crate) fn v4_in(ip: Ipv4Addr, base: [u8; 4], prefix: u32) -> bool {
    let mask = u32::MAX.checked_shl(32 - prefix).unwrap_or(0);
    u32::from(ip) & mask == u32::from(Ipv4Addr::from(base)) & mask
}

/// Whether `ip` is inside `base/prefix` (`prefix` in `0..=128`).
pub(crate) fn v6_in(ip: Ipv6Addr, base: u128, prefix: u32) -> bool {
    let mask = u128::MAX.checked_shl(128 - prefix).unwrap_or(0);
    u128::from(ip) & mask == base & mask
}

#[cfg(test)]
#[path = "_cidr.test.rs"]
mod tests;
