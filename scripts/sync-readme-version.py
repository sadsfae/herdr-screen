#!/usr/bin/env python3
"""Pin README.md install commands to a released version.

Usage: sync-readme-version.py <tag>   e.g. v0.2.1

Replaces the version embedded in the release download URLs of README.md
with the given tag. Fails loudly if the README has no versioned release URL
or the tag does not look like vX.Y.Z. Exits 0 whether or not anything
changed so callers can rely on `git diff` to detect an update.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

README = Path(__file__).resolve().parent.parent / "README.md"
TAG_RE = re.compile(r"v\d+\.\d+\.\d+")


def main() -> int:
    if len(sys.argv) != 2 or not TAG_RE.fullmatch(sys.argv[1]):
        print(f"usage: {Path(__file__).name} <tag>  (e.g. v0.2.1)", file=sys.stderr)
        return 2
    new = sys.argv[1][1:]

    text = README.read_text()
    match = re.search(r"releases/download/v(\d+\.\d+\.\d+)/", text)
    if match is None:
        print("no versioned release URL found in README.md", file=sys.stderr)
        return 2
    old = match.group(1)

    if old == new:
        print(f"README already at {old}")
        return 0

    updated = text.replace(old, new)
    README.write_text(updated)
    print(f"README pinned {old} -> {new}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
