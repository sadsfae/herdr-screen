#!/usr/bin/env bash
# Build all herdrscreen v0.1.0 release artifacts into ./dist:
#   - herdrscreen-linux-x86_64          (musl static-pie binary)
#   - herdrscreen-0.1.0-1.el8.x86_64.rpm  (+ el9, el10, fc43, fc44)
#   - herdrscreen_0.1.0_amd64.deb
#   - herdrscreen-0.1.0.tar.gz          (source snapshot of HEAD)
#   - SHA256SUMS
set -euo pipefail

cd "$(dirname "$0")/.."
VERSION="0.1.0"
REPO="https://github.com/sadsfae/herdrscreen"
DIST="dist"
ZIG="${ZIG:-/tmp/zig-x86_64-linux-0.16.0/zig}"
CARGO="${CARGO:-cargo}"
if [ -n "${TOOLCHAIN_BIN:-}" ]; then
  export PATH="$TOOLCHAIN_BIN:$PATH"
fi
mkdir -p "$DIST"

echo "== static musl build"
ZIG="$ZIG" \
HERDR_BUILD_CHANNEL=herdrscreen \
LIBGHOSTTY_VT_OPTIMIZE=ReleaseFast \
LIBGHOSTTY_VT_SIMD=true \
  "$CARGO" build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/herdrscreen "$DIST/herdrscreen-linux-x86_64"

echo "== RPMs"
TOP="$PWD/build-rpm"
for tag in el8 el9 el10 fc43 fc44; do
  rm -rf "$TOP"
  mkdir -p "$TOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
  cp "$DIST/herdrscreen-linux-x86_64" "$TOP/SOURCES/herdrscreen"
  cp LICENSE NOTICE README.md CHANGELOG.md "$TOP/SOURCES/"
  cp packaging/herdrscreen.spec "$TOP/SPECS/"
  rpmbuild -bb \
    --define "_topdir $TOP" \
    --define "dist .$tag" \
    "$TOP/SPECS/herdrscreen.spec" >/dev/null
  mv "$TOP/RPMS/x86_64/herdrscreen-$VERSION-1.$tag.x86_64.rpm" "$DIST/"
done

echo "== deb"
packaging/build-deb.sh "$DIST/herdrscreen-linux-x86_64" "$DIST/herdrscreen_${VERSION}_amd64.deb"

echo "== source tarball"
git archive --format=tar.gz -o "$DIST/herdrscreen-$VERSION.tar.gz" HEAD

echo "== checksums"
( cd "$DIST" && sha256sum -b herdrscreen-linux-x86_64 herdrscreen-*.rpm herdrscreen_*.deb herdrscreen-*.tar.gz > SHA256SUMS )
echo "done: $(ls -1 "$DIST")"
