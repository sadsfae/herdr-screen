#!/usr/bin/env bash
# Build <repo>/dist/herdr-screen-<version>-x86_64-1.txz, a Slackware package
# (SlackBuilds.org layout: usr/ + install/slack-desc), with no Slackware
# tooling required: tar -cJf is exactly what makepkg emits.
# Requires: a built static binary at $1 (default: <repo>/target/x86_64-unknown-linux-musl/release/herdr-screen)
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${1:-$REPO/target/x86_64-unknown-linux-musl/release/herdr-screen}"
VERSION="${HERDRSCREEN_VERSION:-0.1.0}"
OUT="${2:-$REPO/dist/herdr-screen-${VERSION}-x86_64-1.txz}"
OUT="$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$STAGE/usr/bin" "$STAGE/usr/share/doc/herdr-screen" "$STAGE/install"
install -Dm755 "$BIN" "$STAGE/usr/bin/herdr-screen"
install -Dm644 "$REPO/LICENSE" "$STAGE/usr/share/doc/herdr-screen/LICENSE"
install -Dm644 "$REPO/NOTICE" "$STAGE/usr/share/doc/herdr-screen/NOTICE"
install -Dm644 "$REPO/README.md" "$STAGE/usr/share/doc/herdr-screen/README.md"
install -Dm644 "$REPO/CHANGELOG.md" "$STAGE/usr/share/doc/herdr-screen/CHANGELOG.md"
cat >"$STAGE/install/slack-desc" <<EOF
herdr-screen: herdr-screen (terminal workspace manager, GNU screen edition)
herdr-screen:
herdr-screen: Hard fork of Herdr with GNU screen keybindings by default.
herdr-screen: Each attached client keeps its own viewed tab; features include
herdr-screen: a screen-style window list and terminal lock. The binary is
herdr-screen: statically linked; openssh, git, and curl are needed at runtime.
herdr-screen:
herdr-screen:
herdr-screen:
herdr-screen:
EOF

(cd "$STAGE" && tar --owner=root --group=root -cJf "$OUT" usr install)
echo "built $OUT"
