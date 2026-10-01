# herdrscreen

<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/RPM-Red%20Hat%20%2F%20Rocky%20%2F%20Alma%20EL8%2F9%2F10%20%2F%20Fedora-red?logo=redhat&logoColor=white" alt="RPM packages for Red Hat based distributions" /></a>
<a href="https://github.com/sadsfae/herdrscreen/releases"><img src="https://img.shields.io/badge/DEB-Debian%20%2F%20Ubuntu-blue?logo=debian&logoColor=white" alt="Debian packages for Debian and Ubuntu" /></a>
<a href="https://github.com/sadsfae/herdrscreen/blob/main/packaging/AUR/PKGBUILD"><img src="https://img.shields.io/badge/AUR-Arch%20Linux-1793d1?logo=archlinux&logoColor=white" alt="AUR package for Arch Linux" /></a>

herdrscreen is a GNU screen edition hard fork of [Herdr](https://github.com/herdrdev/herdr)
(Apache-2.0, (C) the Herdr Project contributors), a terminal workspace manager for AI coding
agents. It keeps Herdr's engine and adds:

- GNU screen keybindings by default: prefix is `ctrl+a` (set `prefix = "ctrl+b"` for the
  tmux style), and `ctrl+a ctrl+a` toggles back to the last focused tab
  (`keys.last_tab = "prefix+prefix"`).
- Screen-style overview and lock: `ctrl+a "` lists every tab across workspaces (mouse or
  arrows select, enter focuses), and `ctrl+a ctrl+x` locks the terminal via
  `keys.lock_command` (default `loginctl lock-session`).
- No self-update: `herdrscreen update` is stubbed and version/manifest checks are off. Install
  updates from the [releases page](https://github.com/sadsfae/herdrscreen/releases): RPMs for
  EL8/EL9/EL10 and Fedora, a deb for Debian/Ubuntu, and an AUR package.
- Config and session state stay in Herdr's usual paths (`~/.config/herdr`,
  `~/.local/state/herdr`), so herdrscreen is a drop-in replacement for existing Herdr setups.
- No curl to bash: native packages for Red Hat, Debian, and Arch Linux based distributions.

Licensing and attribution: Apache-2.0. See [LICENSE](LICENSE), [NOTICE](NOTICE), and
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) (QUADS-derived, not upstream's).

The content below is the upstream Herdr README.

---

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">herdr.dev</a> · <a href="#install">install</a> · <a href="https://herdr.dev/docs/quick-start/">quick start</a> · <a href="https://herdr.dev/docs/">docs</a>
</p>

<p align="center">
  English · <a href="README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 license" /></a>
  <a href="https://github.com/herdrdev/herdr/releases"><img src="https://img.shields.io/github/downloads/herdrdev/herdr/total?labelColor=333333&color=666666" alt="total GitHub release downloads" /></a>
  <a href="https://github.com/herdrdev/herdr/stargazers"><img src="https://img.shields.io/github/stars/herdrdev/herdr?labelColor=333333&color=666666&logo=github" alt="GitHub stars" /></a>
  <a href="https://github.com/herdrdev/herdr/releases/latest"><img src="https://img.shields.io/github/v/release/herdrdev/herdr?label=release&labelColor=333333&color=666666" alt="latest stable release" /></a>
  <a href="https://formulae.brew.sh/formula/herdr"><img src="https://img.shields.io/homebrew/v/herdr?label=homebrew&labelColor=333333&color=666666" alt="Homebrew version" /></a>
  <a href="https://x.com/herdrdev"><img src="https://img.shields.io/badge/follow-%40herdrdev-000000?logo=x&logoColor=white" alt="follow @herdrdev on X" /></a>
</p>

---

https://github.com/user-attachments/assets/043ec09f-4bdd-41d5-aee0-8fda6b83e267

**the runtime your coding agents live on.**

- **detach without stopping work** — herdr keeps terminals running in a background server when you close the client or lose your SSH connection. after a server or machine restart, herdr restores the saved layout and can resume supported agent sessions; the original processes do not survive. [session state →](https://herdr.dev/docs/session-state/)
- **several machines, one window** — keep local work and saved ssh machines together, with a combined agent list and independent reconnects. [remote machines →](https://herdr.dev/docs/connecting-machines/)
- **never hunt for the stuck one** — every pane is marked working, blocked, or idle. when an agent stops and needs an answer, herdr says so.
- **agent-native** — agents drive herdr through the cli and socket api: they can spawn panes, prompt each other, and wait until another agent is genuinely blocked. [agent skill →](https://herdr.dev/docs/agent-skill/)
- **runs what you already run** — claude code, codex, cursor, opencode, grok and the rest. herdr doesn't wrap or replace them; it owns their terminals.
- **keyboard and mouse, both first-class** — tmux-style prefix keys *and* click, drag, split. pick per moment, not per tool.
- **plugins** — extend panes and workflows. [browse the marketplace →](https://herdr.dev/plugins/)
- **one rust binary, no electron** — runs in whatever terminal you already use.

---

## install

```bash
curl -fsSL https://herdr.dev/install.sh | sh
```

or `brew install herdr` · `mise use -g herdr` · windows: `powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"` · [endpoint-protected Windows](https://herdr.dev/docs/windows-beta/) · [binaries](https://github.com/herdrdev/herdr/releases)

then start it where the work lives:

```bash
herdr
```

run your agents, split panes, walk away. `ctrl+b q` detaches, `herdr` reattaches. [quick start →](https://herdr.dev/docs/quick-start/)

## docs

everything lives at [herdr.dev/docs](https://herdr.dev/docs/): [quick start](https://herdr.dev/docs/quick-start/) · [concepts](https://herdr.dev/docs/concepts/) · [supported agents](https://herdr.dev/docs/agents/) · [keyboard](https://herdr.dev/docs/keyboard/) · [configuration](https://herdr.dev/docs/configuration/) · [session state](https://herdr.dev/docs/session-state/) · [connecting machines](https://herdr.dev/docs/connecting-machines/) · [remote](https://herdr.dev/docs/persistence-remote/) · [integrations](https://herdr.dev/docs/integrations/) · [plugins](https://herdr.dev/docs/plugins/) · [socket api](https://herdr.dev/docs/socket-api/)

## thanks

every past sponsor and backer is listed in [SPONSORS.md](./SPONSORS.md) — thank you 🐑

enterprise / partnership: hey@herdr.dev

## agent instructions

if you are an ai agent helping with this repository, read [`AGENTS.md`](./AGENTS.md) before making changes and read [`CONTRIBUTING.md`](./CONTRIBUTING.md) before opening issues or PRs.

## development

```bash
git clone https://github.com/herdrdev/herdr
cd herdr
cargo build --release

just test        # unit tests
just check       # formatting, tests, and maintenance checks
```

## license

Herdr is licensed under the [Apache License 2.0](LICENSE).
