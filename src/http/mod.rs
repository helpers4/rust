// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! HTTP header value helpers on plain text: no dependency on an HTTP crate.

mod bearer_token;

pub use bearer_token::bearer_token;
