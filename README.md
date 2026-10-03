# herdr-screen

<a href="https://github.com/sadsfae/herdr-screen/releases"><img src="https://img.shields.io/badge/RPM-Red%20Hat%20%2F%20Rocky%20%2F%20Alma%20EL8%2F9%2F10%20%2F%20Fedora-red?logo=redhat&logoColor=white" alt="RPM packages for Red Hat based distributions" /></a>
<a href="https://github.com/sadsfae/herdr-screen/releases"><img src="https://img.shields.io/badge/DEB-Debian%20%2F%20Ubuntu-blue?logo=debian&logoColor=white" alt="Debian packages for Debian and Ubuntu" /></a>
<a href="https://github.com/sadsfae/herdr-screen/blob/main/packaging/AUR/PKGBUILD"><img src="https://img.shields.io/badge/AUR-Arch%20Linux-1793d1?logo=archlinux&logoColor=white" alt="AUR package for Arch Linux" /></a>
<a href="https://github.com/sadsfae/herdr-screen/releases"><img src="https://img.shields.io/badge/Slackware-txz-2eb8e6?logo=slackware&logoColor=white" alt="Slackware package" /></a>
<a href="https://github.com/sadsfae/herdr-screen/releases"><img src="https://img.shields.io/badge/FreeBSD-pkg-AB2B28?logo=freebsd&logoColor=white" alt="FreeBSD package" /></a>

herdr-screen is a GNU screen edition hard fork of [Herdr](https://github.com/herdrdev/herdr)
(Apache-2.0, (C) the Herdr Project contributors), a terminal workspace manager for AI coding
agents. It keeps Herdr's engine and adds:

## Why?

- herdr with more GNU screen-style features for old salty dogs
- More future features around server/infra management while retaining agentic focus

## Features

herdr-screen is Herdr plus a GNU screen workstyle. The engine, socket API, and
agent model are upstream Herdr; what this fork adds is the screen-style
ergonomics and infra tooling. In short:

- GNU screen keybindings by default: prefix is `ctrl+a` (set `prefix = "ctrl+b"` for the
  tmux style)
  - `ctrl+a ctrl+a` toggles back to the last focused tab (`keys.last_tab = "prefix+prefix"`)
  - `ctrl+a ctrl+x` locks the terminal via `keys.lock_command` (default
    `loginctl lock-session`)
- Screen-style window list (`ctrl+a "`)
  - every tab across every workspace, grouped under its workspace header instead of
    repeating the workspace on each row
  - collapsible headers: `▼` expanded, `▶` collapsed, enter on a header reveals or
    hides its child tabs
  - `/` filters tab titles across workspaces (esc leaves search first)
- Screen-style multi-attach: every connected client keeps its own viewed tab, so one
  client switching tabs never moves another (tmux moves everyone). Socket CLI focus
  (`herdr-screen tab focus`) still moves the whole session.
- No self-update: `herdr-screen update` is stubbed and version/manifest checks are off. Install
  updates from the [releases page](https://github.com/sadsfae/herdr-screen/releases): RPMs for
  EL8/EL9/EL10 and Fedora 42-44, a deb for Debian/Ubuntu, an AUR package, a Slackware txz, and a
  FreeBSD pkg.
- Config and session state stay in Herdr's usual paths (`~/.config/herdr`,
  `~/.local/state/herdr`), so herdr-screen is a drop-in replacement for existing Herdr setups.
- `herdr-screen logs <service> [more...]` opens a dedicated logs workspace with one pane per
  service, each tailing `journalctl -fu <svc>` (use `--file <path>` to tail a log file).
  Define named watchers in `logs-services.toml` (or via `herdr-screen logs-define
  <name> <command>`) so a name tails an arbitrary command or file instead of a systemd unit.
- `herdr-screen layout export [--file layout.json]` writes a workspace/tab layout as a portable
  JSON description; `herdr-screen layout apply <file>` reloads it into a new tab or workspace,
  so you can reuse a layout as a template on another client or machine.
- No curl to bash: native packages for Red Hat, Debian, Arch, Slackware, and FreeBSD.

[Install](#install) · [RPM](#rpm) · [Deb](#deb) · [AUR](#aur) · [Slackware](#slackware) · [FreeBSD](#freebsd)

[About](#about)

[Quick Start Usage](#quick-start-usage)

[Docs](#docs)

Licensing and attribution: Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

https://github.com/user-attachments/assets/043ec09f-4bdd-41d5-aee0-8fda6b83e267

## Install

Download the matching package for your OS from the
[releases page](https://github.com/sadsfae/herdr-screen/releases), or install straight from
the commands below. No curl to bash and no self-update: to update, install the new package
when a release is cut.

### RPM

<img src="assets/redhat.svg" alt="RPM" width="28" />

Fedora:

```bash
sudo dnf install https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen-0.2.3-1.fc43.x86_64.rpm
```

RHEL / Rocky / AlmaLinux 8, 9, or 10 (use the matching `el8`, `el9`, or `el10` asset):

```bash
sudo dnf install https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen-0.2.3-1.el9.x86_64.rpm
```

### Deb

<img src="assets/debian.svg" alt="Deb" width="28" />

Debian / Ubuntu:

```bash
wget https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen_0.2.3_amd64.deb
sudo apt install ./herdr-screen_0.2.3_amd64.deb
```

### AUR

<img src="assets/arch.svg" alt="AUR" width="28" />

Arch Linux (the PKGBUILD ships in-repo; build it until it lands on the AUR):

```bash
git clone --depth 1 https://github.com/sadsfae/herdr-screen
cd herdr-screen/packaging/AUR
makepkg -si
```

### Slackware

<img src="assets/slackware.svg" alt="Slackware" width="28" />

```bash
wget https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen-0.2.3-x86_64-1.txz
installpkg herdr-screen-0.2.3-x86_64-1.txz
```

### FreeBSD

<img src="assets/freebsd.svg" alt="FreeBSD" width="28" />

```bash
pkg add https://github.com/sadsfae/herdr-screen/releases/latest/download/herdr-screen-0.2.3.pkg
```

then start it where the work lives:

```bash
herdr-screen
```

run your agents, split panes, walk away. `ctrl+a q` detaches, `herdr-screen` reattaches. [quick start →](https://herdr.dev/docs/quick-start/)

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

## Quick Start Usage

The most common tasks, in roughly the order you'll reach for them.

### Create a workspace and start work

```bash
herdr-screen workspace create                    # new empty workspace
herdr-screen workspace create --label "api" --cwd ~/src/api
```

A workspace holds a set of tabs and panes. `herdr-screen` (no args) launches or
reattaches to the persistent session.

### Remove a workspace

```bash
herdr-screen workspace list                         # find the id
herdr-screen workspace close <workspace_id>         # close it and free its panes
```

### Define and open a log watcher workspace

Log watchers are defined in **`~/.config/herdr/logs-services.toml`** (one
`"name" = "command"` entry per watcher). Reference definitions for `nginx`,
`postgres`, `ssh`, and `firewall` are seeded automatically. Override or add
your own:

```bash
herdr-screen logs-define myapp tail -F /var/log/myapp.log
herdr-screen logs-define nginx sudo journalctl -fu nginx -n 100
herdr-screen logs-define list                     # show all definitions
```

Open a dedicated logs workspace with one pane per watcher:

```bash
herdr-screen logs nginx postgres                  # tails each in its own pane
herdr-screen logs --file /var/log/app.log         # tail a file path directly
```

### Split panes vertical or horizontal, and revert

`ctrl+a v` splits vertically (left/right), `ctrl+a -` splits horizontally
(top/bottom). To collapse a split back to one pane, close the unwanted panes
with `ctrl+a x` (close pane) or `ctrl+a X` (close tab). `ctrl+a r` enters
resize mode to adjust a split. See `herdr-screen pane --help` for the CLI
equivalents.

### Window list

`ctrl+a "` shows every tab across every workspace, grouped under its workspace
header. Headers are collapsible (`▼`/`▶`), and `/` filters tab titles.

## Docs

everything lives at [herdr.dev/docs](https://herdr.dev/docs/): [quick start](https://herdr.dev/docs/quick-start/) · [concepts](https://herdr.dev/docs/concepts/) · [supported agents](https://herdr.dev/docs/agents/) · [keyboard](https://herdr.dev/docs/keyboard/) · [configuration](https://herdr.dev/docs/configuration/) · [session state](https://herdr.dev/docs/session-state/) · [connecting machines](https://herdr.dev/docs/connecting-machines/) · [remote](https://herdr.dev/docs/persistence-remote/) · [integrations](https://herdr.dev/docs/integrations/) · [plugins](https://herdr.dev/docs/plugins/) · [socket api](https://herdr.dev/docs/socket-api/)
