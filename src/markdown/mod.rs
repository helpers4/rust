// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for writing Markdown safely: escaping, links, code, quotes, tables and heading anchors.
//!
//! Everything here produces Markdown text; nothing parses it. The functions make untrusted text and
//! awkward characters (backticks, pipes, brackets, parentheses) come out literally.

mod blockquote;
mod code_block;
mod escape;
mod heading_slug;
mod inline_code;
mod link;
mod table;

pub use blockquote::blockquote;
pub use code_block::code_block;
pub use escape::escape;
pub use heading_slug::heading_slug;
pub use inline_code::inline_code;
pub use link::link;
pub use table::table;
