#!/usr/bin/env bash
# Build <repo>/dist/herdr-screen-<version>.pkg, a FreeBSD pkg(8) package
# (xz-compressed tar with +MANIFEST/+DESC, prefix /usr/local), on any Linux
# host. Requires: a built static binary at $1 (default: <repo>/target/x86_64-unknown-linux-musl/release/herdr-screen)
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${1:-$REPO/target/x86_64-unknown-linux-musl/release/herdr-screen}"
VERSION="${HERDRSCREEN_VERSION:-0.1.0}"
OUT="${2:-$REPO/dist/herdr-screen-${VERSION}.pkg}"
OUT="$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$STAGE/usr/local/bin" "$STAGE/usr/local/share/doc/herdr-screen" "$STAGE/usr/local/share/licenses/herdr-screen"
install -Dm755 "$BIN" "$STAGE/usr/local/bin/herdr-screen"
install -Dm644 "$REPO/LICENSE" "$STAGE/usr/local/share/licenses/herdr-screen/LICENSE"
install -Dm644 "$REPO/NOTICE" "$STAGE/usr/local/share/doc/herdr-screen/NOTICE"
install -Dm644 "$REPO/README.md" "$STAGE/usr/local/share/doc/herdr-screen/README.md"
install -Dm644 "$REPO/CHANGELOG.md" "$STAGE/usr/local/share/doc/herdr-screen/CHANGELOG.md"

FILES_JSON=""
for path in /usr/local/bin/herdr-screen \
  /usr/local/share/licenses/herdr-screen/LICENSE \
  /usr/local/share/doc/herdr-screen/NOTICE \
  /usr/local/share/doc/herdr-screen/README.md \
  /usr/local/share/doc/herdr-screen/CHANGELOG.md; do
  digest=$(sha256sum "$STAGE$path" | awk '{print $1}')
  FILES_JSON="$FILES_JSON\"$path\": \"$digest\", "
done
FILES_JSON="${FILES_JSON%, }"
FLATSIZE=$(du -sk "$STAGE/usr/local" | cut -f1)

cat >"$STAGE/+MANIFEST" <<EOF
{
  "name": "herdr-screen",
  "origin": "sysutils/herdr-screen",
  "version": "$VERSION",
  "comment": "Terminal workspace manager for AI coding agents (GNU screen edition)",
  "maintainer": "sadsfae <sadsfae@users.noreply.github.com>",
  "www": "https://github.com/sadsfae/herdr-screen",
  "prefix": "/usr/local",
  "categories": ["sysutils"],
  "licenses": ["APACHE20"],
  "arch": "amd64",
  "pkgtype": "pkg",
  "flatsize": $((FLATSIZE * 1024)),
  "files": { $FILES_JSON }
}
EOF
cat >"$STAGE/+DESC" <<EOF
herdr-screen is a GNU screen edition hard fork of Herdr: a terminal workspace
manager for AI coding agents. Defaults to GNU screen keybindings; each
attached client keeps its own viewed tab. Statically linked; openssh, git,
and curl are needed at runtime.
EOF

(cd "$STAGE" && tar --owner=0 --group=0 -cJf "$OUT" +MANIFEST +DESC usr)
echo "built $OUT"
