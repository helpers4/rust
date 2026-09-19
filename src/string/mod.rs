// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! String manipulation and formatting helpers.

mod _words;
mod camel_case;
mod capitalize;
mod dedent;
mod escape_html;
mod kebab_case;
mod pascal_case;
mod slugify;
mod snake_case;
mod truncate;

pub use camel_case::camel_case;
pub use capitalize::capitalize;
pub use dedent::dedent;
pub use escape_html::escape_html;
pub use kebab_case::kebab_case;
pub use pascal_case::pascal_case;
pub use slugify::slugify;
pub use snake_case::snake_case;
pub use truncate::truncate;
