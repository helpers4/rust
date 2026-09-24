#!/usr/bin/env python3
# This file is part of helpers4.
# Copyright (C) 2025 baxyz
# SPDX-License-Identifier: LGPL-3.0-or-later

"""Coherency checks for the helpers4 (Rust) repository.

Equivalent of typescript's `pnpm coherency`: catches drift between the pieces that must agree by
hand (Cargo.toml features, lib.rs gates, scopes.json, llms.txt, benches) but that no compiler or
test enforces. Exits non-zero on the first category with failures, after printing every failure it
found (not just the first one).

Run: python3 scripts/coherency.py
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
LICENSE_HEADER = (
    "// This file is part of helpers4.\n"
    "// Copyright (C) 2025 baxyz\n"
    "// SPDX-License-Identifier: LGPL-3.0-or-later\n"
)

# Files whose only job is to be included from another file with #[path = "..."] and that never
# carry their own public API: still need the license header, but not an `# Examples` scan.
SIDECAR_SUFFIXES = (".test.rs", ".spec.rs", ".bench.rs")


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def modules() -> list[str]:
    return sorted(p.name for p in SRC.iterdir() if p.is_dir() and (p / "mod.rs").is_file())


def check_license_headers() -> list[str]:
    failures = []
    for path in sorted(SRC.rglob("*.rs")):
        if not read(path).startswith(LICENSE_HEADER):
            failures.append(f"{path.relative_to(ROOT)}: missing or malformed license header")
    return failures


def check_module_wiring(mods: list[str]) -> list[str]:
    failures = []
    cargo_toml = read(ROOT / "Cargo.toml")
    lib_rs = read(ROOT / "src" / "lib.rs")
    scopes = json.loads(read(ROOT / "scopes.json"))
    llms_txt = read(ROOT / "llms.txt")

    default_match = re.search(r'default\s*=\s*\[([^\]]*)\]', cargo_toml)
    default_features = set(re.findall(r'"([^"]+)"', default_match.group(1))) if default_match else set()

    for module in mods:
        if not re.search(rf'^{re.escape(module)}\s*=', cargo_toml, re.MULTILINE):
            failures.append(f"Cargo.toml: no `{module} = [...]` feature")
        if module not in default_features:
            failures.append(f"Cargo.toml: `default` does not include \"{module}\"")
        if not re.search(
            rf'#\[cfg\(feature\s*=\s*"{re.escape(module)}"\)\]\s*\npub mod {re.escape(module)};',
            lib_rs,
        ):
            failures.append(f"src/lib.rs: no `#[cfg(feature = \"{module}\")] pub mod {module};`")
        if module not in scopes:
            failures.append(f"scopes.json: missing \"{module}\"")
        if not re.search(rf'^- {re.escape(module)} [—-]', llms_txt, re.MULTILINE):
            failures.append(f"llms.txt: no `- {module} —` entry under Modules")

        module_dir = SRC / module
        has_bench_source = any(module_dir.glob("*.bench.rs"))
        if has_bench_source:
            bench_file = ROOT / "benches" / f"{module}.rs"
            if not bench_file.is_file():
                failures.append(f"benches/{module}.rs: missing, but src/{module} has *.bench.rs files")
            stanza = next(
                (
                    s for s in cargo_toml.split("\n\n")
                    if s.strip().startswith("[[bench]]") and f'name = "{module}"' in s
                ),
                None,
            )
            if stanza is None:
                failures.append(f"Cargo.toml: no `[[bench]] name = \"{module}\"` entry")
            else:
                req = re.search(r'required-features\s*=\s*\[([^\]]*)\]', stanza)
                if not req or f'"{module}"' not in req.group(1):
                    failures.append(
                        f"Cargo.toml: `[[bench]] name = \"{module}\"` missing `required-features = [\"{module}\"]`"
                    )
    return failures


DECL_RE = re.compile(r'^pub\s+(fn|struct|enum|type|const|trait)\s+([A-Za-z_][A-Za-z0-9_]*)')

# A doctest needs something to run. These kinds are supporting items documented through the
# helper that returns or uses them (CONTRIBUTING.md, "Export it"): a `# Examples` block on top
# would just repeat the one on the type/function that matters. Error enums are exempted by name
# regardless of kind, since some are structs rather than enums.
EXEMPT_KINDS = {"const", "type"}


def exported_items(mod_rs: Path) -> list[tuple[str, str]]:
    """Returns (source-file-stem, exported-name) for every `pub use <stem>::<name>;` in mod.rs."""
    items = []
    for line in read(mod_rs).splitlines():
        m = re.match(r'pub use ([a-z_][a-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*);', line.strip())
        if m:
            items.append((m.group(1), m.group(2)))
    return items


def has_examples_section(source: str, name: str) -> str | None:
    """None if `name`'s doc block has `# Examples` (or is exempt); otherwise a reason string."""
    if name.endswith("Error"):
        return None
    lines = source.splitlines()
    for i, line in enumerate(lines):
        m = DECL_RE.match(line)
        if m and m.group(2) == name:
            if m.group(1) in EXEMPT_KINDS:
                return None
            doc_lines = []
            j = i - 1
            while j >= 0 and (lines[j].lstrip().startswith("///") or lines[j].lstrip().startswith("#[")):
                if lines[j].lstrip().startswith("///"):
                    doc_lines.append(lines[j])
                j -= 1
            if not doc_lines:
                return "no rustdoc comment"
            doc = "\n".join(reversed(doc_lines))
            if "# Examples" not in doc:
                return "rustdoc has no `# Examples` section"
            return None
    return "declaration not found (renamed without updating mod.rs?)"


def check_examples(mods: list[str]) -> list[str]:
    failures = []
    for module in mods:
        mod_rs = SRC / module / "mod.rs"
        for stem, name in exported_items(mod_rs):
            source_file = SRC / module / f"{stem}.rs"
            if not source_file.is_file():
                failures.append(f"src/{module}/mod.rs: `pub use {stem}::{name}` but {source_file.name} does not exist")
                continue
            reason = has_examples_section(read(source_file), name)
            if reason:
                failures.append(f"src/{module}/{stem}.rs: `{name}` — {reason}")
    return failures


def check_api_since(mods: list[str]) -> list[str]:
    """Every exported item has an `api-since.json` entry. `api-since.json` only gains entries
    when someone runs `update-api-since.py <version>` while preparing a release (the version a
    helper ships in is not known before then), so this is **not** part of the default checks that
    run on every PR — job-lint.yml would fail for the ordinary PR that adds a helper. It runs only
    at release time, from `release.yml`, right before publishing (`--api-since` flag)."""
    api_since_path = ROOT / "api-since.json"
    if not api_since_path.is_file():
        return ["api-since.json: file does not exist"]
    api_since = json.loads(read(api_since_path))
    failures = []
    for module in mods:
        for _stem, name in exported_items(SRC / module / "mod.rs"):
            if f"{module}::{name}" not in api_since:
                failures.append(
                    f"api-since.json: no entry for `{module}::{name}` — "
                    f"run `python3 scripts/update-api-since.py <version>` while preparing the release"
                )
    return failures


def main() -> int:
    mods = modules()
    checks = [
        ("license headers", check_license_headers()),
        ("module wiring", check_module_wiring(mods)),
        ("rustdoc `# Examples`", check_examples(mods)),
    ]
    if "--api-since" in sys.argv[1:]:
        checks.append(("api-since.json coverage", check_api_since(mods)))

    total = 0
    for title, failures in checks:
        if failures:
            print(f"\n{title}: {len(failures)} problem(s)")
            for f in failures:
                print(f"  - {f}")
            total += len(failures)

    if total:
        print(f"\ncoherency: {total} problem(s) found")
        return 1

    print(f"coherency: OK ({len(mods)} modules)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
