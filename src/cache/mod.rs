// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Caches and stores whose entries expire. The clock is always passed in, never read.

mod expiring_map;

pub use expiring_map::ExpiringMap;
pub use expiring_map::ExpiringSet;
