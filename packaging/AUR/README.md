# Arch package (no AUR account required)

`PKGBUILD` builds herdr-screen from source for Arch Linux and derivatives
(AlterLinux, CachyOS, EndeavourOS, Manjaro, ...). zig 0.16.0 is required by
the vendored terminal; Arch ships exactly `extra/zig` = 0.16.0 and
`extra/rust` >= 1.99.

## Install from a release (fastest)

Every version-bump release carries
`herdr-screen-<version>-1-x86_64.pkg.tar.zst`, built by the same PKGBUILD in
the `packages` workflow:

```bash
cd /tmp
curl -fsSLO https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen-0.2.5-1-x86_64.pkg.tar.zst
sudo pacman -U herdr-screen-0.2.5-1-x86_64.pkg.tar.zst
```

## Build it yourself

```bash
pacman -S --needed base-devel rust zig   # zig must be 0.16.0
makepkg -si
```

The build takes a while (Rust release build of the full workspace); `makepkg`
downloads the crate sources and the vendored `ghostty-vt` is built with zig.

## Notes

- `pkgver()` follows the release cadence (0.2.4 from a `v0.2.4-<sha>` tag);
  the workflow pins the exact version with `PKGVER_OVERRIDE` so the artifact
  name matches the release it is attached to.
- The source is the `main` archive snapshot (rolling); release tags pin an
  identical tree plus a `SHA256SUMS` file.
- To publish on AUR later: register the package on aur.archlinux.org, then in
  this directory run `makepkg --printsrcinfo > .SRCINFO` and push `PKGBUILD`
  and `.SRCINFO` to `ssh://aur@aur.archlinux.org/herdr-screen.git`.
