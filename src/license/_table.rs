// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The licenses this module knows: SPDX identifier, name, family. Not re-exported.

use super::Category;
use super::Category::{NetworkCopyleft, Permissive, PublicDomain, StrongCopyleft, WeakCopyleft};

pub(crate) const TABLE: [(&str, &str, Category); 42] = [
    ("0BSD", "BSD Zero Clause License", Permissive),
    (
        "AGPL-3.0-only",
        "GNU Affero General Public License v3.0 only",
        NetworkCopyleft,
    ),
    (
        "AGPL-3.0-or-later",
        "GNU Affero General Public License v3.0 or later",
        NetworkCopyleft,
    ),
    ("Apache-2.0", "Apache License 2.0", Permissive),
    ("Artistic-2.0", "Artistic License 2.0", Permissive),
    ("BlueOak-1.0.0", "Blue Oak Model License 1.0.0", Permissive),
    (
        "BSD-2-Clause",
        "BSD 2-Clause \"Simplified\" License",
        Permissive,
    ),
    (
        "BSD-3-Clause",
        "BSD 3-Clause \"New\" or \"Revised\" License",
        Permissive,
    ),
    ("BSL-1.0", "Boost Software License 1.0", Permissive),
    (
        "CC0-1.0",
        "Creative Commons Zero v1.0 Universal",
        PublicDomain,
    ),
    (
        "CDDL-1.0",
        "Common Development and Distribution License 1.0",
        WeakCopyleft,
    ),
    ("EPL-1.0", "Eclipse Public License 1.0", WeakCopyleft),
    ("EPL-2.0", "Eclipse Public License 2.0", WeakCopyleft),
    (
        "GPL-2.0-only",
        "GNU General Public License v2.0 only",
        StrongCopyleft,
    ),
    (
        "GPL-2.0-or-later",
        "GNU General Public License v2.0 or later",
        StrongCopyleft,
    ),
    (
        "GPL-3.0-only",
        "GNU General Public License v3.0 only",
        StrongCopyleft,
    ),
    (
        "GPL-3.0-or-later",
        "GNU General Public License v3.0 or later",
        StrongCopyleft,
    ),
    ("ISC", "ISC License", Permissive),
    (
        "LGPL-2.1-only",
        "GNU Lesser General Public License v2.1 only",
        WeakCopyleft,
    ),
    (
        "LGPL-2.1-or-later",
        "GNU Lesser General Public License v2.1 or later",
        WeakCopyleft,
    ),
    (
        "LGPL-3.0-only",
        "GNU Lesser General Public License v3.0 only",
        WeakCopyleft,
    ),
    (
        "LGPL-3.0-or-later",
        "GNU Lesser General Public License v3.0 or later",
        WeakCopyleft,
    ),
    ("MIT", "MIT License", Permissive),
    ("MIT-0", "MIT No Attribution", Permissive),
    ("MPL-2.0", "Mozilla Public License 2.0", WeakCopyleft),
    (
        "NCSA",
        "University of Illinois/NCSA Open Source License",
        Permissive,
    ),
    ("PostgreSQL", "PostgreSQL License", Permissive),
    ("Python-2.0", "Python License 2.0", Permissive),
    (
        "SSPL-1.0",
        "Server Side Public License, v 1",
        NetworkCopyleft,
    ),
    ("UPL-1.0", "Universal Permissive License v1.0", Permissive),
    ("Unicode-3.0", "Unicode License v3", Permissive),
    ("Unlicense", "The Unlicense", PublicDomain),
    (
        "WTFPL",
        "Do What The F*ck You Want To Public License",
        Permissive,
    ),
    ("Zlib", "zlib License", Permissive),
    (
        "BSD-3-Clause-Clear",
        "BSD 3-Clause Clear License",
        Permissive,
    ),
    (
        "EUPL-1.2",
        "European Union Public License 1.2",
        StrongCopyleft,
    ),
    ("MS-PL", "Microsoft Public License", Permissive),
    ("MS-RL", "Microsoft Reciprocal License", WeakCopyleft),
    ("OpenSSL", "OpenSSL License", Permissive),
    (
        "W3C",
        "W3C Software Notice and License (2002-12-31)",
        Permissive,
    ),
    ("X11", "X11 License", Permissive),
    ("Apache-1.1", "Apache License 1.1", Permissive),
];

/// Deprecated SPDX identifiers that are still everywhere, and what replaced them.
pub(crate) const DEPRECATED: [(&str, &str); 10] = [
    ("AGPL-3.0", "AGPL-3.0-only"),
    ("AGPL-3.0+", "AGPL-3.0-or-later"),
    ("GPL-2.0", "GPL-2.0-only"),
    ("GPL-2.0+", "GPL-2.0-or-later"),
    ("GPL-3.0", "GPL-3.0-only"),
    ("GPL-3.0+", "GPL-3.0-or-later"),
    ("LGPL-2.1", "LGPL-2.1-only"),
    ("LGPL-2.1+", "LGPL-2.1-or-later"),
    ("LGPL-3.0", "LGPL-3.0-only"),
    ("LGPL-3.0+", "LGPL-3.0-or-later"),
];

#[cfg(test)]
#[path = "_table.test.rs"]
mod tests;
