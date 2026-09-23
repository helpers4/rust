// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The basic CSS color names. Not re-exported.

/// The 17 basic CSS color keywords and their sRGB values.
pub(crate) const NAMED: [(&str, [u8; 3]); 17] = [
    ("black", [0, 0, 0]),
    ("silver", [192, 192, 192]),
    ("gray", [128, 128, 128]),
    ("white", [255, 255, 255]),
    ("maroon", [128, 0, 0]),
    ("red", [255, 0, 0]),
    ("purple", [128, 0, 128]),
    ("fuchsia", [255, 0, 255]),
    ("green", [0, 128, 0]),
    ("lime", [0, 255, 0]),
    ("olive", [128, 128, 0]),
    ("yellow", [255, 255, 0]),
    ("navy", [0, 0, 128]),
    ("blue", [0, 0, 255]),
    ("teal", [0, 128, 128]),
    ("aqua", [0, 255, 255]),
    ("orange", [255, 165, 0]),
];
