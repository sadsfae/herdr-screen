#!/usr/bin/env bash
# Build all herdr-screen release artifacts into ./dist. The version comes from
# Cargo.toml unless VERSION is set explicitly (the packages workflow passes it).
#   - herdr-screen-linux-x86_64          (musl static-pie binary)
#   - herdr-screen-<version>-1.el8.x86_64.rpm  (+ el9, el10, fc42, fc43, fc44)
#   - herdr-screen_<version>_amd64.deb
#   - herdr-screen-<version>.tar.gz      (source snapshot of HEAD)
#   - SHA256SUMS
set -euo pipefail

cd "$(dirname "$0")/.."
VERSION="${VERSION:-$(head -1 RELEASE_VERSION | tr -d '[:space:]')}"
REPO="https://github.com/sadsfae/herdr-screen"
DIST="dist"
ZIG="${ZIG:-/tmp/zig-x86_64-linux-0.16.0/zig}"
CARGO="${CARGO:-cargo}"
if [ -n "${TOOLCHAIN_BIN:-}" ]; then
  export PATH="$TOOLCHAIN_BIN:$PATH"
fi
mkdir -p "$DIST"

echo "== static musl build"
ZIG="$ZIG" \
HERDR_BUILD_CHANNEL=herdr-screen \
LIBGHOSTTY_VT_OPTIMIZE=ReleaseFast \
LIBGHOSTTY_VT_SIMD=true \
  "$CARGO" build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/herdr-screen "$DIST/herdr-screen-linux-x86_64"

echo "== RPMs"
TOP="$PWD/build-rpm"
for tag in el8 el9 el10 fc42 fc43 fc44; do
  rm -rf "$TOP"
  mkdir -p "$TOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
  cp "$DIST/herdr-screen-linux-x86_64" "$TOP/SOURCES/herdr-screen"
  cp LICENSE NOTICE README.md CHANGELOG.md "$TOP/SOURCES/"
  cp packaging/herdr-screen.spec "$TOP/SPECS/"
  rpmbuild -bb \
    --define "_topdir $TOP" \
    --define "dist .$tag" \
    --define "version $VERSION" \
    "$TOP/SPECS/herdr-screen.spec" >/dev/null
  mv "$TOP/RPMS/x86_64/herdr-screen-$VERSION-1.$tag.x86_64.rpm" "$DIST/"
done

echo "== deb"
HERDRSCREEN_VERSION="$VERSION" packaging/build-deb.sh "$DIST/herdr-screen-linux-x86_64" "$DIST/herdr-screen_${VERSION}_amd64.deb"

echo "== slackware txz"
HERDRSCREEN_VERSION="$VERSION" packaging/build-slack.sh "$DIST/herdr-screen-linux-x86_64" "$DIST/herdr-screen-${VERSION}-x86_64-1.txz"

echo "== freebsd pkg"
HERDRSCREEN_VERSION="$VERSION" packaging/build-freebsd.sh "$DIST/herdr-screen-linux-x86_64" "$DIST/herdr-screen-${VERSION}.pkg"

echo "== source tarball"
git archive --format=tar.gz -o "$DIST/herdr-screen-$VERSION.tar.gz" HEAD

echo "== checksums"
( cd "$DIST" && sha256sum -b herdr-screen-linux-x86_64 herdr-screen-*.rpm herdr-screen_*.deb herdr-screen-*.txz herdr-screen-*.pkg herdr-screen-*.tar.gz > SHA256SUMS )
echo "done: $(ls -1 "$DIST")"
