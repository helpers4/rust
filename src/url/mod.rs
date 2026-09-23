// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! URLs without a dependency: percent-encoding, query strings, a parser and reference resolution.
//!
//! `percent_encode` / `percent_decode` handle one component, `parse_query` / `build_query` turn a query
//! string into pairs and back, `join_path` glues path pieces, and [`Url`] parses a whole URL (RFC 3986)
//! and resolves relative references against it. Malformed input is an error, never a silent fix.

mod build_query;
mod join_path;
mod parse_query;
mod parsed_url;
mod percent_decode;
mod percent_decode_error;
mod percent_encode;
mod url_error;

pub use build_query::build_query;
pub use join_path::join_path;
pub use parse_query::parse_query;
pub use parsed_url::Url;
pub use percent_decode::percent_decode;
pub use percent_decode_error::PercentDecodeError;
pub use percent_encode::percent_encode;
pub use url_error::UrlError;
