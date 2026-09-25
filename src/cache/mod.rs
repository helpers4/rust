// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Caches and stores: entries that expire with a passed-in clock ([`ExpiringMap`]), and a
//! capacity-bounded map that evicts by recency instead ([`LruCache`]). Neither ever reads a
//! clock or the system allocator's notion of "recent" behind the caller's back.

mod expiring_map;
mod lru_cache;

pub use expiring_map::ExpiringMap;
pub use expiring_map::ExpiringSet;
pub use lru_cache::LruCache;
