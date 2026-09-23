// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_prerelease::{check_identifiers, cmp_prerelease, number};
use super::{ParseVersionError, Version};
use std::cmp::Ordering;

/// A version requirement such as `^1.2`, `>=1.0, <2.0` or `1.*`: which versions are acceptable.
///
/// The syntax and the rules are Cargo's. A requirement is comma-separated comparators that must
/// all match. Operators: `=`, `>`, `>=`, `<`, `<=`, `~` (tilde: patch-level changes, or minor when
/// the patch is omitted), `^` (caret: changes that keep the left-most non-zero number), and none,
/// which means caret. `1`, `1.2` and `1.2.3` may be partial; `*`, `x` and `X` are wildcards
/// (`1.*`, `1.2.x`, or `*` alone for anything). A pre-release version only matches a
/// requirement that names a pre-release of the same `MAJOR.MINOR.PATCH`.
///
/// # Examples
///
/// ```
/// use helpers4::version::{Version, VersionReq};
///
/// let req = VersionReq::parse("^1.2")?;
/// assert!(req.matches(&Version::parse("1.9.0")?));
/// assert!(!req.matches(&Version::parse("2.0.0")?));
/// assert!(!req.matches(&Version::parse("1.1.9")?));
///
/// let range = VersionReq::parse(">=1.0, <1.5")?;
/// assert!(range.matches(&Version::parse("1.4.9")?));
/// # Ok::<(), helpers4::version::ParseVersionError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq {
    comparators: Vec<Comparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
    Caret,
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Comparator {
    op: Op,
    major: u64,
    minor: Option<u64>,
    patch: Option<u64>,
    pre: Option<String>,
}

impl VersionReq {
    /// Parses a requirement.
    ///
    /// # Arguments
    ///
    /// - `text` - The requirement, such as `"^1.2"`, `">=1.0, <2.0"` or `"*"`.
    ///
    /// # Errors
    ///
    /// A [`ParseVersionError`]: [`Empty`](ParseVersionError::Empty) for an empty text,
    /// [`InvalidComparator`](ParseVersionError::InvalidComparator) for a malformed comparator
    /// (unknown operator, a wildcard with an operator other than `=`, a pre-release on a partial
    /// version, build metadata), or the number and identifier errors of [`Version::parse`].
    pub fn parse(text: &str) -> Result<Self, ParseVersionError> {
        if text.trim().is_empty() {
            return Err(ParseVersionError::Empty);
        }
        let mut comparators = Vec::new();
        for part in text.split(',') {
            let part = part.trim();
            if part == "*" {
                continue;
            }
            comparators.push(Comparator::parse(part)?);
        }
        Ok(Self { comparators })
    }

    /// Whether `version` satisfies every comparator (and the pre-release rule).
    ///
    /// # Arguments
    ///
    /// - `version` - The version to check.
    ///
    /// # Returns
    ///
    /// `true` when the version is acceptable.
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        self.comparators.iter().all(|c| c.matches(version))
            && (!version.is_prerelease()
                || self
                    .comparators
                    .iter()
                    .any(|c| c.allows_prerelease_of(version)))
    }
}

impl Comparator {
    fn parse(text: &str) -> Result<Self, ParseVersionError> {
        let (op, rest) = split_operator(text);
        let rest = rest.trim_start();
        let rest = rest.strip_prefix(['v', 'V']).unwrap_or(rest);
        if rest.is_empty() || rest.contains('+') {
            return Err(ParseVersionError::InvalidComparator);
        }
        let (core, pre) = match rest.split_once('-') {
            Some((core, pre)) => {
                check_identifiers(pre, true)?;
                (core, Some(pre.to_string()))
            }
            None => (rest, None),
        };
        let mut parts = core.splitn(3, '.');
        let major = number("major", parts.next().unwrap_or(""))?;
        let minor = part("minor", parts.next())?;
        let patch = part("patch", parts.next())?;
        let wildcard = minor == Part::Wildcard || patch == Part::Wildcard;
        // A wildcard may only end the version (`1.*`, `1.2.*`), and only with `=` or no operator.
        let number_after_wildcard = minor == Part::Wildcard && matches!(patch, Part::Number(_));
        if number_after_wildcard || (wildcard && !matches!(op, None | Some(Op::Exact))) {
            return Err(ParseVersionError::InvalidComparator);
        }
        let (minor, patch) = (minor.number(), patch.number());
        if pre.is_some() && (minor.is_none() || patch.is_none()) {
            return Err(ParseVersionError::InvalidComparator);
        }
        let op = if wildcard {
            Op::Wildcard
        } else {
            op.unwrap_or(Op::Caret)
        };
        Ok(Self {
            op,
            major,
            minor,
            patch,
            pre,
        })
    }

    fn matches(&self, v: &Version) -> bool {
        match self.op {
            Op::Exact => self.exact(v),
            Op::Greater => self.greater(v),
            Op::GreaterEq => self.exact(v) || self.greater(v),
            Op::Less => self.less(v),
            Op::LessEq => self.exact(v) || self.less(v),
            Op::Tilde => self.tilde(v),
            Op::Caret => self.caret(v),
            Op::Wildcard => self.wildcard(v),
        }
    }

    fn pre_order(&self, v: &Version) -> Ordering {
        cmp_prerelease(v.pre(), self.pre.as_deref())
    }

    fn exact(&self, v: &Version) -> bool {
        if v.major() != self.major {
            return false;
        }
        if self.minor.is_some_and(|minor| v.minor() != minor) {
            return false;
        }
        if self.patch.is_some_and(|patch| v.patch() != patch) {
            return false;
        }
        v.pre() == self.pre.as_deref()
    }

    fn greater(&self, v: &Version) -> bool {
        if v.major() != self.major {
            return v.major() > self.major;
        }
        let Some(minor) = self.minor else {
            return false;
        };
        if v.minor() != minor {
            return v.minor() > minor;
        }
        let Some(patch) = self.patch else {
            return false;
        };
        if v.patch() != patch {
            return v.patch() > patch;
        }
        self.pre_order(v) == Ordering::Greater
    }

    fn less(&self, v: &Version) -> bool {
        if v.major() != self.major {
            return v.major() < self.major;
        }
        let Some(minor) = self.minor else {
            return false;
        };
        if v.minor() != minor {
            return v.minor() < minor;
        }
        let Some(patch) = self.patch else {
            return false;
        };
        if v.patch() != patch {
            return v.patch() < patch;
        }
        self.pre_order(v) == Ordering::Less
    }

    fn tilde(&self, v: &Version) -> bool {
        if v.major() != self.major {
            return false;
        }
        if self.minor.is_some_and(|minor| v.minor() != minor) {
            return false;
        }
        if let Some(patch) = self.patch {
            if v.patch() != patch {
                return v.patch() > patch;
            }
        }
        self.pre_order(v) != Ordering::Less
    }

    fn caret(&self, v: &Version) -> bool {
        if v.major() != self.major {
            return false;
        }
        let Some(minor) = self.minor else {
            return true;
        };
        let Some(patch) = self.patch else {
            return if self.major > 0 {
                v.minor() >= minor
            } else {
                v.minor() == minor
            };
        };
        if self.major > 0 {
            if v.minor() != minor {
                return v.minor() > minor;
            }
            if v.patch() != patch {
                return v.patch() > patch;
            }
        } else if minor > 0 {
            if v.minor() != minor {
                return false;
            }
            if v.patch() != patch {
                return v.patch() > patch;
            }
        } else if v.minor() != minor || v.patch() != patch {
            return false;
        }
        self.pre_order(v) != Ordering::Less
    }

    fn wildcard(&self, v: &Version) -> bool {
        v.major() == self.major && self.minor.is_none_or(|minor| v.minor() == minor)
    }

    fn allows_prerelease_of(&self, v: &Version) -> bool {
        self.pre.is_some()
            && self.major == v.major()
            && self.minor == Some(v.minor())
            && self.patch == Some(v.patch())
    }
}

/// Splits a leading operator off a comparator.
fn split_operator(text: &str) -> (Option<Op>, &str) {
    for (symbol, op) in [
        ("<=", Op::LessEq),
        (">=", Op::GreaterEq),
        ("<", Op::Less),
        (">", Op::Greater),
        ("=", Op::Exact),
        ("~", Op::Tilde),
        ("^", Op::Caret),
    ] {
        if let Some(rest) = text.strip_prefix(symbol) {
            return (Some(op), rest);
        }
    }
    (None, text)
}

/// A minor or patch part of a comparator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Absent,
    Wildcard,
    Number(u64),
}

impl Part {
    fn number(self) -> Option<u64> {
        match self {
            Self::Number(n) => Some(n),
            Self::Absent | Self::Wildcard => None,
        }
    }
}

fn part(component: &'static str, text: Option<&str>) -> Result<Part, ParseVersionError> {
    match text {
        None => Ok(Part::Absent),
        Some("*" | "x" | "X") => Ok(Part::Wildcard),
        Some(text) => number(component, text).map(Part::Number),
    }
}

#[cfg(test)]
#[path = "version_req.test.rs"]
mod tests;

#[cfg(test)]
#[path = "version_req.spec.rs"]
mod spec;
