// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Detecting CI environments and reporting pipeline status.
//!
//! `detect`, `is_ci` and `is_pull_request` take the environment as a lookup function instead of reading it,
//! so they are as easy to test as to use (`&|name| std::env::var(name).ok()`). [`Status`], `overall` and
//! `render_report` turn job results into the Markdown summary of a PR comment.

mod detect;
mod is_ci;
mod is_pull_request;
mod overall;
mod provider;
mod render_report;
mod status;

pub use detect::detect;
pub use is_ci::is_ci;
pub use is_pull_request::is_pull_request;
pub use overall::overall;
pub use provider::Provider;
pub use render_report::render_report;
pub use status::Status;
