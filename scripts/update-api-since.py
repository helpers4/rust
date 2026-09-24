#!/usr/bin/env python3
# This file is part of helpers4.
# Copyright (C) 2025 baxyz
# SPDX-License-Identifier: LGPL-3.0-or-later

"""Update api-since.json: which released version first shipped each public item.

No "Since 0.x" annotation is written in the source (see AGENTS.md): it is computed from the public
API diff between releases, with `cargo public-api` (needs the nightly toolchain: `rustup toolchain
install nightly`).

Usage, when preparing a release's version-bump PR (after bumping Cargo.toml, alongside
`git cliff --tag vX.Y.Z -o CHANGELOG.md`):

    python3 scripts/update-api-since.py X.Y.Z

Only new top-level items (`helpers4::<module>::<Name>`, matching what `mod.rs` re-exports) get an
entry; a changed signature on an existing name keeps its original "since" version, and a method or
trait impl is not tracked on its own (it is not a `pub use` target and coherency.py does not check
it either).

One-time backfill across every existing tag (already run to seed the file):

    python3 scripts/update-api-since.py --backfill
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
API_SINCE = ROOT / "api-since.json"

ITEM_KIND_RE = re.compile(r'^pub (fn|struct|enum|trait|const|type)\s+(.*)')


def run(*args: str) -> str:
    # cargo-public-api occasionally fails to build the rustdoc JSON for one side of a diff when
    # it is one of several ran back-to-back against the same target/ directory; a bare retry
    # clears it, so this is treated as flakiness rather than a real error.
    last: subprocess.CalledProcessError | None = None
    for _ in range(3):
        try:
            return subprocess.run(args, cwd=ROOT, check=True, capture_output=True, text=True).stdout
        except subprocess.CalledProcessError as e:
            last = e
    assert last is not None
    print(last.stdout, file=sys.stderr)
    print(last.stderr, file=sys.stderr)
    raise last


def top_level_path(rest: str) -> list[str]:
    """`rest` is everything after `pub <kind> `; returns its `::`-separated path segments, with
    generics, a `const`'s `: Type` annotation, and a `type` alias' `= ...` right-hand side
    stripped first (all of which can themselves contain `::` and would otherwise look like more
    path segments)."""
    path_part = rest.split("(")[0].split("=")[0]
    while "<" in path_part:
        stripped = re.sub(r'<[^<>]*>', '', path_part)
        if stripped == path_part:
            break
        path_part = stripped
    path_part = re.sub(r'(?<!:):(?!:).*$', '', path_part)  # a const's `: Type` annotation
    return [s for s in path_part.strip().split("::") if s]


def tags() -> list[str]:
    out = subprocess.run(
        ["git", "tag", "-l", "v*"], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout
    versions = [t.strip() for t in out.splitlines() if t.strip()]
    return sorted(versions, key=lambda v: [int(p) for p in v.lstrip("v").split(".")])


def top_level_item(line: str) -> tuple[str, str] | None:
    """(module, name) if `line` (one line of `cargo public-api` output, `+`-prefix already
    stripped if it had one) declares a top-level item; `None` for impls, methods, fields,
    variants and module declarations."""
    line = re.sub(r'^#\[[^\]]*\]\s*', '', line)
    m = ITEM_KIND_RE.match(line)
    if not m:
        return None
    _, rest = m.groups()
    segments = top_level_path(rest)
    if len(segments) == 3 and segments[0] == "helpers4":
        return (segments[1], segments[2])
    return None


def parse_added_items(diff_output: str) -> set[tuple[str, str]]:
    """Returns {(module, name)} for top-level items in the "Added items" section of a diff."""
    lines = diff_output.splitlines()
    try:
        start = next(i for i, l in enumerate(lines) if l.startswith("Added items")) + 2
    except StopIteration:
        return set()
    items = set()
    for line in lines[start:]:
        if line.startswith("===") or (line.strip() == "" and items):
            continue
        if not line.startswith("+"):
            continue
        item = top_level_item(line[1:])
        if item:
            items.add(item)
    return items


def list_items(rev: str) -> set[tuple[str, str]]:
    """Every top-level item in the public API at `rev` (used for the first release: no baseline).

    `cargo public-api` only lists the *current working tree*; there is no revision argument for
    listing (only for `diff`), so this checks `rev` out into a throwaway worktree.
    """
    with tempfile.TemporaryDirectory(prefix="h4-api-since-") as tmp:
        subprocess.run(
            ["git", "worktree", "add", "--detach", tmp, rev],
            cwd=ROOT, check=True, capture_output=True, text=True,
        )
        try:
            output = subprocess.run(
                ["cargo", "public-api", "--all-features", "-s", "-s"],
                cwd=tmp, check=True, capture_output=True, text=True,
            ).stdout
        finally:
            subprocess.run(
                ["git", "worktree", "remove", "--force", tmp],
                cwd=ROOT, check=True, capture_output=True, text=True,
            )
    items = set()
    for line in output.splitlines():
        item = top_level_item(line)
        if item:
            items.add(item)
    return items


def load() -> dict[str, str]:
    return json.loads(API_SINCE.read_text()) if API_SINCE.is_file() else {}


def save(data: dict[str, str]) -> None:
    ordered = dict(sorted(data.items()))
    API_SINCE.write_text(json.dumps(ordered, indent=2) + "\n")


def backfill() -> int:
    data: dict[str, str] = {}
    all_tags = tags()
    if not all_tags:
        print("No vX.Y.Z tags found.", file=sys.stderr)
        return 1

    first = all_tags[0]
    print(f"Listing the public API at {first} (first release)...")
    for module, name in list_items(first):
        data[f"{module}::{name}"] = first.lstrip("v")

    for prev, cur in zip(all_tags, all_tags[1:]):
        print(f"Diffing {prev}..{cur}...")
        diff = run("cargo", "public-api", "diff", f"{prev}..{cur}", "--all-features", "-s", "-s")
        for module, name in parse_added_items(diff):
            key = f"{module}::{name}"
            data.setdefault(key, cur.lstrip("v"))

    print(f"Diffing {all_tags[-1]}..HEAD (unreleased)...")
    diff = run("cargo", "public-api", "diff", f"{all_tags[-1]}..HEAD", "--all-features", "-s", "-s")
    unreleased = parse_added_items(diff) - {
        (k.split("::")[0], k.split("::")[1]) for k in data
    }
    if unreleased:
        print("Unreleased additions since the last tag (not written; run in normal mode once tagged):")
        for module, name in sorted(unreleased):
            print(f"  - {module}::{name}")

    save(data)
    print(f"api-since.json: {len(data)} items from {len(all_tags)} releases.")
    return 0


def update_for_release(version: str) -> int:
    all_tags = tags()
    if not all_tags:
        print("No vX.Y.Z tags found; run --backfill first.", file=sys.stderr)
        return 1
    last = all_tags[-1]
    print(f"Diffing {last}..HEAD...")
    diff = run("cargo", "public-api", "diff", f"{last}..HEAD", "--all-features", "-s", "-s")
    data = load()
    added = 0
    for module, name in sorted(parse_added_items(diff)):
        key = f"{module}::{name}"
        if key not in data:
            data[key] = version
            added += 1
            print(f"  + {key} -> {version}")
    save(data)
    print(f"api-since.json: {added} new item(s) tagged {version}.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", nargs="?", help="Version being prepared, e.g. 0.0.7")
    parser.add_argument("--backfill", action="store_true", help="Rebuild from every existing tag")
    args = parser.parse_args()

    if args.backfill:
        return backfill()
    if not args.version:
        parser.error("pass a version (e.g. 0.0.7) or --backfill")
    return update_for_release(args.version)


if __name__ == "__main__":
    sys.exit(main())
