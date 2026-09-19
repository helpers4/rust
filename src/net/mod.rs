// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Network helpers on `std::net` and plain text: no I/O, no resolution.

mod _cidr;
mod error;
mod is_public_ip;
mod is_valid_hostname;

pub use error::HostnameError;
pub use is_public_ip::is_public_ip;
pub use is_valid_hostname::is_valid_hostname;
