// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Dotenv (`.env`) helpers working on plain text: no file or process-environment access, so
//! they are deterministic and easy to test. Read the file yourself, edit the content here, and
//! write it back.

mod _line;
mod error;
mod get;
mod get_bool;
mod get_duration;
mod get_int;
mod get_list;
mod parse;
mod remove;
mod set;

pub use error::InvalidKeyError;
pub use get::get;
pub use get_bool::get_bool;
pub use get_duration::get_duration;
pub use get_int::get_int;
pub use get_list::get_list;
pub use parse::parse;
pub use remove::remove;
pub use set::set;
