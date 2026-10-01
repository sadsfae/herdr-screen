# herdrscreen

<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/RPM-Red%20Hat%20%2F%20Rocky%20%2F%20Alma%20EL8%2F9%2F10%20%2F%20Fedora-red?logo=redhat&logoColor=white" alt="RPM packages for Red Hat based distributions" /></a>
<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/DEB-Debian%20%2F%20Ubuntu-blue?logo=debian&logoColor=white" alt="Debian packages for Debian and Ubuntu" /></a>
<a href="https://github.com/sadsfae/herdrscreen/blob/main/packaging/AUR/PKGBUILD"><img src="https://img.shields.io/badge/AUR-Arch%20Linux-1793d1?logo=archlinux&logoColor=white" alt="AUR package for Arch Linux" /></a>
<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/Slackware-txz-2eb8e6?logo=slackware&logoColor=white" alt="Slackware package" /></a>
<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/FreeBSD-pkg-AB2B28?logo=freebsd&logoColor=white" alt="FreeBSD package" /></a>

herdrscreen is a GNU screen edition hard fork of [Herdr](https://github.com/herdrdev/herdr)
(Apache-2.0, (C) the Herdr Project contributors), a terminal workspace manager for AI coding
agents. It keeps Herdr's engine and adds:

- GNU screen keybindings by default: prefix is `ctrl+a` (set `prefix = "ctrl+b"` for the
  tmux style), and `ctrl+a ctrl+a` toggles back to the last focused tab
  (`keys.last_tab = "prefix+prefix"`).
- Screen-style overview and lock: `ctrl+a "` lists every tab across workspaces (mouse or
  arrows select, enter focuses), and `ctrl+a ctrl+x` locks the terminal via
  `keys.lock_command` (default `loginctl lock-session`).
- Screen-style multi-attach: every connected client keeps its own viewed tab, so one
  client switching tabs never moves another (tmux moves everyone). Socket CLI focus
  (`herdr tab focus`) still moves the whole session.
- No self-update: `herdrscreen update` is stubbed and version/manifest checks are off. Install
  updates from the [releases page](https://github.com/sadsfae/herdrscreen/releases): RPMs for
  EL8/EL9/EL10 and Fedora, a deb for Debian/Ubuntu, an AUR package, a Slackware txz, and a
  FreeBSD pkg.
- Config and session state stay in Herdr's usual paths (`~/.config/herdr`,
  `~/.local/state/herdr`), so herdrscreen is a drop-in replacement for existing Herdr setups.
- No curl to bash: native packages for Red Hat, Debian, Arch, Slackware, and FreeBSD.

Licensing and attribution: Apache-2.0. See [LICENSE](LICENSE), [NOTICE](NOTICE), and
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Table of Contents

- [Install](#install)
  - [RPM](#rpm)
  - [Deb](#deb)
  - [AUR](#aur)
  - [Slackware](#slackware)
  - [FreeBSD](#freebsd)
- [About](#about)
- [Upstream herdr Docs](#upstream-herdr-docs)
- [Thanks](#thanks)

---

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

https://github.com/user-attachments/assets/043ec09f-4bdd-41d5-aee0-8fda6b83e267

## Install

Download the matching package for your OS from the
[releases page](https://github.com/sadsfae/herdrscreen/releases), or install straight from
the commands below. No curl to bash and no self-update: to update, install the new package
when a release is cut.

### RPM

Fedora:

```bash
sudo dnf install ./herdrscreen-0.2.0-1.fc43.x86_64.rpm
```

RHEL / Rocky / AlmaLinux 8, 9, or 10 (use the matching `el8`, `el9`, or `el10` asset):

```bash
sudo dnf install ./herdrscreen-0.2.0-1.el9.x86_64.rpm
```

### Deb

Debian / Ubuntu:

```bash
sudo apt install ./herdrscreen_0.2.0_amd64.deb
```

### AUR

Arch Linux (the PKGBUILD ships in-repo; build it until it lands on the AUR):

```bash
git clone --depth 1 https://github.com/sadsfae/herdrscreen
cd herdrscreen/packaging/AUR
makepkg -si
```

### Slackware

```bash
installpkg herdrscreen-0.2.0-x86_64-1.txz
```

### FreeBSD

```bash
pkg add ./herdrscreen-0.2.0.pkg
```

then start it where the work lives:

```bash
herdr
```

run your agents, split panes, walk away. `ctrl+a q` detaches, `herdr` reattaches. [quick start →](https://herdr.dev/docs/quick-start/)

## About

**the runtime your coding agents live on.**

- **detach without stopping work** — herdr keeps terminals running in a background server when you close the client or lose your SSH connection. after a server or machine restart, herdr restores the saved layout and can resume supported agent sessions; the original processes do not survive. [session state →](https://herdr.dev/docs/session-state/)
- **several machines, one window** — keep local work and saved ssh machines together, with a combined agent list and independent reconnects. [remote machines →](https://herdr.dev/docs/connecting-machines/)
- **never hunt for the stuck one** — every pane is marked working, blocked, or idle. when an agent stops and needs an answer, herdr says so.
- **agent-native** — agents drive herdr through the cli and socket api: they can spawn panes, prompt each other, and wait until another agent is genuinely blocked. [agent skill →](https://herdr.dev/docs/agent-skill/)
- **runs what you already run** — claude code, codex, cursor, opencode, grok and the rest. herdr doesn't wrap or replace them; it owns their terminals.
- **keyboard and mouse, both first-class** — tmux-style prefix keys *and* click, drag, split. pick per moment, not per tool.
- **plugins** — extend panes and workflows. [browse the marketplace →](https://herdr.dev/plugins/)
- **one rust binary, no electron** — runs in whatever terminal you already use.

## Upstream herdr Docs

everything lives at [herdr.dev/docs](https://herdr.dev/docs/): [quick start](https://herdr.dev/docs/quick-start/) · [concepts](https://herdr.dev/docs/concepts/) · [supported agents](https://herdr.dev/docs/agents/) · [keyboard](https://herdr.dev/docs/keyboard/) · [configuration](https://herdr.dev/docs/configuration/) · [session state](https://herdr.dev/docs/session-state/) · [connecting machines](https://herdr.dev/docs/connecting-machines/) · [remote](https://herdr.dev/docs/persistence-remote/) · [integrations](https://herdr.dev/docs/integrations/) · [plugins](https://herdr.dev/docs/plugins/) · [socket api](https://herdr.dev/docs/socket-api/)

## Thanks

every past sponsor and backer is listed in [SPONSORS.md](./SPONSORS.md) — thank you 🐑

enterprise / partnership: hey@herdr.dev
