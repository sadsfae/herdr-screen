#!/usr/bin/env python3
"""Pin README.md install commands to the current release version.

The README install URLs use Github's `releases/latest/download/<asset>`
resolver, so they always point at the newest release. The asset filenames
embed the release version, so this script rewrites every `herdr-screen-<ver>` /
`herdr-screen_<ver>` reference to match RELEASE_VERSION (the fork's release
cadence, independent of the runtime version in Cargo.toml). Fails loudly if
RELEASE_VERSION has no version. Exits 0 whether or not anything changed so
callers can rely on `git diff` to detect an update.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
README = ROOT / "README.md"
RELEASE = ROOT / "RELEASE_VERSION"


def release_version() -> str:
    match = re.search(r"^\s*([^\s]+)", RELEASE.read_text(), re.MULTILINE)
    if match is None:
        print("no version found in RELEASE_VERSION", file=sys.stderr)
        raise SystemExit(2)
    return match.group(1)


def main() -> int:
    version = release_version()

    text = README.read_text()
    # Migrate any old `releases/download/vX.Y.Z/` URLs to the latest resolver so
    # install docs always fetch the newest release regardless of tag.
    text = re.sub(
        r"releases/download/v\d+\.\d+\.\d+/",
        "releases/latest/download/",
        text,
    )
    # Rewrite every embedded asset version (`herdr-screen-0.2.2.rpm`,
    # `herdr-screen_0.2.2.deb`, ...) to the current release version.
    new_text = re.sub(
        r"(herdr-screen[-_])\d+\.\d+\.\d+",
        lambda m: f"{m.group(1)}{version}",
        text,
    )
    if new_text == text:
        print(f"README already at {version}")
        return 0

    README.write_text(new_text)
    print(f"README pinned to {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
