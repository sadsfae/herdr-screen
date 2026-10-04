#!/usr/bin/env python3
"""Compute the next fork release version from conventional commit subjects.

The packages workflow bumps the published package version like semver: a
breaking change (type!:) bumps the major, feat bumps the minor, fix/perf
bumps the patch, and everything else leaves the version unchanged (and the
workflow skips publishing a release). Generated from the commit subjects
between the previous release tag and HEAD.

Usage:
  next_version.py --base 0.2.3 --range v0.2.2..HEAD
  next_version.py --base 0.2.3 --subjects "fix: reload config" "feat: logs"
  next_version.py --tag-version v0.2.3-7802b16
"""

from __future__ import annotations

import argparse
import re
import sys

# Reuse the single conventional-commit convention from the CI validator so
# the bump rules cannot drift from the validation rules.
try:
    from scripts.conventional_commits import ALLOWED_TYPES, SUBJECT_RE, git_subjects
except ImportError:  # direct execution from scripts/
    from conventional_commits import ALLOWED_TYPES, SUBJECT_RE, git_subjects

TAG_RE = re.compile(r"^v?(\d+)\.(\d+)\.(\d+)(?:-[0-9a-f]{7,})?$")


def bump_level(subject: str) -> int:
    """Return the semver bump for one commit subject: 3 major, 2 minor,
    1 patch, 0 none."""
    match = SUBJECT_RE.match(subject)
    if match is None:
        return 0
    kind = match.group("kind")
    if kind not in ALLOWED_TYPES:
        return 0
    if re.match(r"^[a-z]+(?:\([^)]+\))?!:", subject):
        return 3
    if kind == "feat":
        return 2
    if kind in ("fix", "perf"):
        return 1
    return 0


def next_version(base: str, level: int) -> str:
    """Apply a semver bump (0 none, 1 patch, 2 minor, 3 major) to base."""
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)$", base)
    if match is None:
        raise ValueError(f"invalid base version: {base!r}")
    major, minor, patch = (int(group) for group in match.groups())
    if level >= 3:
        return f"{major + 1}.0.0"
    if level == 2:
        return f"{major}.{minor + 1}.0"
    if level == 1:
        return f"{major}.{minor}.{patch + 1}"
    return base


def tag_version(tag: str) -> str:
    """Strip the v prefix and per-push sha suffix from a release tag."""
    match = TAG_RE.match(tag)
    if match is None:
        raise ValueError(f"invalid release tag: {tag!r}")
    return f"{match.group(1)}.{match.group(2)}.{match.group(3)}"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", help="Current version, e.g. 0.2.3")
    parser.add_argument("--subjects", nargs="*", help="Commit subjects")
    parser.add_argument("--range", dest="rev_range", help="git rev range")
    parser.add_argument("--tag-version", dest="parse_tag")
    args = parser.parse_args()

    if args.parse_tag:
        try:
            print(tag_version(args.parse_tag))
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 2
        return 0

    if not args.base:
        parser.error("--base is required unless --tag-version is given")

    subjects = list(args.subjects or [])
    if args.rev_range:
        subjects.extend(git_subjects(args.rev_range))

    level = max((bump_level(subject) for subject in subjects), default=0)
    try:
        print(next_version(args.base, level))
    except ValueError as error:
        print(str(error), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
