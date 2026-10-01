#!/usr/bin/env bash
# Build <repo>/dist/herdrscreen-<version>-x86_64-1.txz, a Slackware package
# (SlackBuilds.org layout: usr/ + install/slack-desc), with no Slackware
# tooling required: tar -cJf is exactly what makepkg emits.
# Requires: a built static binary at $1 (default: <repo>/target/x86_64-unknown-linux-musl/release/herdrscreen)
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${1:-$REPO/target/x86_64-unknown-linux-musl/release/herdrscreen}"
VERSION="${HERDRSCREEN_VERSION:-0.1.0}"
OUT="${2:-$REPO/dist/herdrscreen-${VERSION}-x86_64-1.txz}"
OUT="$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$STAGE/usr/bin" "$STAGE/usr/share/doc/herdrscreen" "$STAGE/install"
install -Dm755 "$BIN" "$STAGE/usr/bin/herdrscreen"
install -Dm644 "$REPO/LICENSE" "$STAGE/usr/share/doc/herdrscreen/LICENSE"
install -Dm644 "$REPO/NOTICE" "$STAGE/usr/share/doc/herdrscreen/NOTICE"
install -Dm644 "$REPO/README.md" "$STAGE/usr/share/doc/herdrscreen/README.md"
install -Dm644 "$REPO/CHANGELOG.md" "$STAGE/usr/share/doc/herdrscreen/CHANGELOG.md"
cat >"$STAGE/install/slack-desc" <<EOF
herdrscreen: herdrscreen (terminal workspace manager, GNU screen edition)
herdrscreen:
herdrscreen: Hard fork of Herdr with GNU screen keybindings by default.
herdrscreen: Each attached client keeps its own viewed tab; features include
herdrscreen: a screen-style window list and terminal lock. The binary is
herdrscreen: statically linked; openssh, git, and curl are needed at runtime.
herdrscreen:
herdrscreen:
herdrscreen:
herdrscreen:
EOF

(cd "$STAGE" && tar --owner=root --group=root -cJf "$OUT" usr install)
echo "built $OUT"
