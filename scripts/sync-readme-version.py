#!/usr/bin/env python3
"""Pin README.md install commands to the current release version.

The README install URLs use Github's `releases/latest/download/<asset>`
resolver, so they always point at the newest release. The asset filenames
embed the release version, so this script rewrites every `herdr-screen-<ver>` /
`herdr-screen_<ver>` reference to match the release version passed on the
command line (or RELEASE_VERSION, the fork's release cadence, when no
`--version` is given). With `--version`, RELEASE_VERSION is updated to match
what was actually published. Fails loudly if the version is missing. Exits 0
whether or not anything changed so callers can rely on `git diff` to detect
an update.
"""

from __future__ import annotations

import argparse
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
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--version",
        help="Published release version; also written to RELEASE_VERSION",
    )
    args = parser.parse_args()

    version = args.version or release_version()

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
    if new_text != text:
        README.write_text(new_text)
        print(f"README pinned to {version}")
    else:
        print(f"README already at {version}")

    if args.version:
        current = release_version()
        if current != version:
            RELEASE.write_text(f"{version}\n")
            print(f"RELEASE_VERSION updated from {current} to {version}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
