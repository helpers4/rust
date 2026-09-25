// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! String manipulation and formatting helpers.

mod _words;
mod camel_case;
mod capitalize;
mod constant_case;
mod dedent;
mod escape_html;
mod escape_regex;
mod excerpt;
mod indent;
mod kebab_case;
mod levenshtein_distance;
mod levenshtein_similarity;
mod pascal_case;
#[cfg(feature = "string-diacritics")]
mod remove_diacritics;
mod slugify;
mod snake_case;
mod squish;
mod title_case;
mod truncate;
mod unescape_html;
mod words;

pub use camel_case::camel_case;
pub use capitalize::capitalize;
pub use constant_case::constant_case;
pub use dedent::dedent;
pub use escape_html::escape_html;
pub use escape_regex::escape_regex;
pub use excerpt::excerpt;
pub use indent::indent;
pub use kebab_case::kebab_case;
pub use levenshtein_distance::levenshtein_distance;
pub use levenshtein_similarity::levenshtein_similarity;
pub use pascal_case::pascal_case;
#[cfg(feature = "string-diacritics")]
pub use remove_diacritics::remove_diacritics;
pub use slugify::slugify;
pub use snake_case::snake_case;
pub use squish::squish;
pub use title_case::title_case;
pub use truncate::truncate;
pub use unescape_html::unescape_html;
pub use words::words;
