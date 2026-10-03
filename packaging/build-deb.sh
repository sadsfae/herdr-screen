#!/usr/bin/env bash
# Build herdr-screen_<version>_amd64.deb with nothing but binutils (ar/tar).
# Requires: a built static binary at $1 (default: <repo>/target/x86_64-unknown-linux-musl/release/herdr-screen)
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${1:-$REPO/target/x86_64-unknown-linux-musl/release/herdr-screen}"
VERSION="${HERDRSCREEN_VERSION:-0.1.0}"
OUT="${2:-$REPO/dist/herdr-screen_${VERSION}_amd64.deb}"
OUT="$(cd "$(dirname "$OUT")" && pwd)/$(basename "$OUT")"
ROOT="$REPO/dist/deb-root"

rm -rf "$ROOT"
mkdir -p "$ROOT/DEBIAN" "$ROOT/usr/bin" "$ROOT/usr/share/doc/herdr-screen"
install -Dm755 "$BIN" "$ROOT/usr/bin/herdr-screen"
install -Dm644 "$REPO/LICENSE" "$ROOT/usr/share/doc/herdr-screen/copyright"
install -Dm644 "$REPO/NOTICE" "$ROOT/usr/share/doc/herdr-screen/NOTICE"
install -Dm644 "$REPO/README.md" "$ROOT/usr/share/doc/herdr-screen/README.md"
install -Dm644 "$REPO/CHANGELOG.md" "$ROOT/usr/share/doc/herdr-screen/changelog"

SIZE=$(du -sk "$ROOT/usr" | cut -f1)
cat >"$ROOT/DEBIAN/control" <<EOF
Package: herdr-screen
Version: $VERSION
Architecture: amd64
Maintainer: sadsfae <sadsfae@users.noreply.github.com>
Installed-Size: $SIZE
Section: misc
Priority: optional
Description: Terminal workspace manager for AI coding agents (GNU screen edition)
 Hard fork of Herdr with GNU screen keybindings by default (prefix ctrl+a,
 ctrl+a ctrl+a toggles the last focused tab). Statically linked; installs no
 dependencies. Requires openssh-client, git, and curl available at runtime.
EOF

(cd "$ROOT/DEBIAN" && tar --format=gnu -czf ../control.tar.gz .)
(cd "$ROOT" && tar --format=gnu -czf data.tar.gz usr)
cd "$ROOT"
printf '2.0\n' > debian-binary
ar r "$OUT" debian-binary control.tar.gz data.tar.gz 2>/dev/null || ar rcs "$OUT" debian-binary control.tar.gz data.tar.gz
rm -rf "$ROOT"
echo "built $OUT"
