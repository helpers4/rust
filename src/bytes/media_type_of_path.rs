// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

const TYPES: &[(&str, &str)] = &[
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("avif", "image/avif"),
    ("svg", "image/svg+xml"),
    ("ico", "image/x-icon"),
    ("bmp", "image/bmp"),
    ("woff", "font/woff"),
    ("woff2", "font/woff2"),
    ("ttf", "font/ttf"),
    ("otf", "font/otf"),
    ("html", "text/html"),
    ("htm", "text/html"),
    ("css", "text/css"),
    ("js", "text/javascript"),
    ("mjs", "text/javascript"),
    ("json", "application/json"),
    ("txt", "text/plain"),
    ("md", "text/markdown"),
    ("xml", "application/xml"),
    ("wasm", "application/wasm"),
    ("pdf", "application/pdf"),
    ("zip", "application/zip"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
    ("ogg", "audio/ogg"),
    ("mp4", "video/mp4"),
    ("webm", "video/webm"),
];

/// Finds the media type of a file from the extension of its path or URL: images, fonts, web
/// documents, scripts and styles, data, audio and video. Case is ignored, and a query string or
/// fragment after the path is skipped. Pass it where a server or a blob needs a `Content-Type`.
///
/// # Arguments
///
/// - `path` - A file path or URL.
///
/// # Returns
///
/// The media type (`"image/webp"`, ...), or `None` for an unknown or missing extension.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::media_type_of_path;
///
/// assert_eq!(media_type_of_path("layers/a@2x.webp"), Some("image/webp"));
/// assert_eq!(media_type_of_path("/fonts/Inter.WOFF2?v=3"), Some("font/woff2"));
/// assert_eq!(media_type_of_path("LICENSE"), None);
/// ```
#[must_use]
pub fn media_type_of_path(path: &str) -> Option<&'static str> {
    let path = path.split(['?', '#']).next().unwrap_or("");
    let name = path.rsplit(['/', '\\']).next().unwrap_or("");
    let (_, extension) = name.rsplit_once('.')?;
    TYPES
        .iter()
        .find(|(known, _)| extension.eq_ignore_ascii_case(known))
        .map(|(_, media_type)| *media_type)
}

#[cfg(test)]
#[path = "media_type_of_path.test.rs"]
mod tests;

#[cfg(test)]
#[path = "media_type_of_path.spec.rs"]
mod spec;
