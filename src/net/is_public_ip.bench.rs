// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::net::is_public_ip;
use std::hint::black_box;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn bench(c: &mut Criterion) {
    let public_v4 = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let private_v4 = IpAddr::V4(Ipv4Addr::new(10, 1, 2, 3));
    let public_v6 = IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111));
    let mapped_v6 = IpAddr::V6(Ipv4Addr::new(169, 254, 169, 254).to_ipv6_mapped());
    c.bench_function("net::is_public_ip/v4-public", |b| {
        b.iter(|| is_public_ip(black_box(public_v4)));
    });
    c.bench_function("net::is_public_ip/v4-private", |b| {
        b.iter(|| is_public_ip(black_box(private_v4)));
    });
    c.bench_function("net::is_public_ip/v6-public", |b| {
        b.iter(|| is_public_ip(black_box(public_v6)));
    });
    c.bench_function("net::is_public_ip/v6-mapped", |b| {
        b.iter(|| is_public_ip(black_box(mapped_v6)));
    });
}
